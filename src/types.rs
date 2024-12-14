use std::fmt::{self, Debug, Display};
use std::ops::{Add, Sub};

/// The frame of reference for a coordinate to be interpreted with.
///
/// This is used to determine how a coordinate should be used.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoordinateFrame {
    /// The coordinate is relative to the world, and is an absolute position.
    ///
    /// As an example, if the world is 100x100x100, and the coordinate is (50, 50, 50),
    /// then *no matter* what we are feeding coordinates to, it will always refer to the center of the world.
    World,

    /// The coordinate is relative to the partition, and is not an absolute position.
    ///
    /// As an example, if the partition is 100x100x100, and the coordinate is (50, 50, 50),
    /// then it will refer to the center of the partition.
    /// However, if the partition is at (100, 100, 100) in the world, then the coordinate will refer to (150, 150, 150) in the world.
    Relative,

    /// The coordinate is not referring to a location within the world, but is instead an index position.
    ///
    /// As an example, assume partitions are 10x10x10, and the coordinate is (2, 2, 2).
    /// If we ask for a partition at that coordinate, we will get the partition at (20, 20, 20).
    /// See that it refers to the partition at this location in indexing, not a location within the partition or world.
    Index,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CoordinateError {
    OutOfBounds,
    InvalidFrame,
}

/// A generic 3D coordinate trait.
///
/// This is used to represent a 3D coordinate.
pub trait Coordinate: Sized + Add + Sub + Debug + Display + PartialEq + Eq {
    type Scalar: num::Num + num::NumCast;
    type Internal: num::Num + num::NumCast;
    fn x(&self) -> Self::Scalar;
    fn y(&self) -> Self::Scalar;
    fn z(&self) -> Self::Scalar;
    fn zero() -> Self;
    fn up() -> Self;
    fn down() -> Self;
    fn left() -> Self;
    fn right() -> Self;
    fn forward() -> Self;
    fn back() -> Self;
    fn from<T: Coordinate>(coord: T) -> Result<Self, CoordinateError>
    where
        Self::Internal: From<T::Scalar>;
}

#[cfg(feature = "big_coordinates")]
type SpatialCoordinateScalar = i64;
#[cfg(not(feature = "big_coordinates"))]
type SpatialCoordinateScalar = i32;

/// A generic 3D coordinate in the world.
#[derive(Clone, Copy)]
pub struct SpatialCoordinate {
    pub x: SpatialCoordinateScalar,
    pub y: SpatialCoordinateScalar,
    pub z: SpatialCoordinateScalar,
}

impl Coordinate for SpatialCoordinate {
    /// The scalar type used for the coordinate.
    type Scalar = SpatialCoordinateScalar;
    /// The internal type used for the coordinate - This helps conversion.
    type Internal = SpatialCoordinateScalar;
    #[inline]
    fn x(&self) -> Self::Scalar {
        self.x
    }
    #[inline]
    fn y(&self) -> Self::Scalar {
        self.y
    }
    #[inline]
    fn z(&self) -> Self::Scalar {
        self.z
    }
    #[inline]
    fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
    #[inline]
    #[allow(dead_code)]
    fn up() -> Self {
        Self { x: 0, y: 1, z: 0 }
    }
    #[inline]
    #[allow(dead_code)]
    fn down() -> Self {
        Self { x: 0, y: -1, z: 0 }
    }
    #[inline]
    #[allow(dead_code)]
    fn left() -> Self {
        Self { x: -1, y: 0, z: 0 }
    }
    #[inline]
    #[allow(dead_code)]
    fn right() -> Self {
        Self { x: 1, y: 0, z: 0 }
    }
    #[inline]
    #[allow(dead_code)]
    fn forward() -> Self {
        Self { x: 0, y: 0, z: 1 }
    }
    #[inline]
    #[allow(dead_code)]
    fn back() -> Self {
        Self { x: 0, y: 0, z: -1 }
    }
    #[inline]
    fn from<T: Coordinate>(coord: T) -> Result<Self, CoordinateError>
    where
        Self::Internal: From<T::Scalar>,
    {
        Ok(Self {
            x: Self::Scalar::from(coord.x()),
            y: Self::Scalar::from(coord.y()),
            z: Self::Scalar::from(coord.z()),
        })
    }
}

impl Add for SpatialCoordinate {
    type Output = SpatialCoordinate;

    fn add(self, other: SpatialCoordinate) -> SpatialCoordinate {
        SpatialCoordinate {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for SpatialCoordinate {
    type Output = SpatialCoordinate;

    fn sub(self, other: SpatialCoordinate) -> SpatialCoordinate {
        SpatialCoordinate {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Debug for SpatialCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Display for SpatialCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for SpatialCoordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for SpatialCoordinate {}

/// An area in the world.
///
/// This is used to represent a volume of space in the world.
pub struct Area {
    pub from: SpatialCoordinate,
    pub to: SpatialCoordinate,
}

impl Area {
    /// Creates a new area with zero volume.
    #[allow(dead_code)]
    pub fn zero() -> Area {
        Area {
            from: SpatialCoordinate::zero(),
            to: SpatialCoordinate::zero(),
        }
    }

    /// Checks if the area contains the given coordinate.
    ///
    /// # Example
    /// ```rust
    /// use minecraft_world::types::{Area, Coordinate};
    ///
    /// let area = Area {
    ///    from: Coordinate::zero(),
    ///   to: Coordinate { x: 10, y: 10, z: 10 }
    /// };
    ///
    /// let a = Coordinate { x: 5, y: 5, z: 5 };
    /// let b = Coordinate { x: 15, y: 15, z: 15 };
    ///
    /// assert!(area.contains(a));
    /// assert!(!area.contains(b));
    /// ```
    #[allow(dead_code)]
    pub fn contains(&self, coord: SpatialCoordinate) -> bool {
        coord.x >= self.from.x
            && coord.x <= self.to.x
            && coord.y >= self.from.y
            && coord.y <= self.to.y
            && coord.z >= self.from.z
            && coord.z <= self.to.z
    }

    /// Offsets the area by the given coordinate.
    #[allow(dead_code)]
    pub fn offset(&mut self, coord: SpatialCoordinate) {
        self.from = self.from + coord;
        self.to = self.to + coord;
    }

    /// Returns the contained volume of the area.
    ///
    /// # Example
    /// ```rust
    /// use minecraft_world::types::{Area, Coordinate};
    ///
    /// let area = Area {
    ///    from: Coordinate::zero(),
    ///   to: Coordinate { x: 10, y: 10, z: 10 }
    /// };
    ///
    /// assert_eq!(area.volume(), 1000);
    /// ```
    #[allow(dead_code)]
    pub fn volume(&self) -> i32 {
        (self.to.x - self.from.x) * (self.to.y - self.from.y) * (self.to.z - self.from.z)
    }
}

impl Debug for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) -> ({})", self.from, self.to)
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) -> ({})", self.from, self.to)
    }
}
