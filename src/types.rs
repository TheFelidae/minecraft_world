use std::fmt::{self, Debug, Display};
use std::ops::{Add, Sub};

/// The frame of reference for a coordinate to be interpreted with.
/// 
/// This is used to determine how a coordinate should be used.
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
    Index
}

/// A scalar value for a coordinate. This is used to represent a single axis of a coordinate.
///
/// If additional size is needed, this can be enabled via the `big_coordinates` feature.
#[cfg(feature = "big_coordinates")]
pub type CoordinateScalar = i64;
#[cfg(not(feature = "big_coordinates"))]
pub type CoordinateScalar = i32;

/// A 3D coordinate in the world.
#[derive(Clone, Copy)]
pub struct Coordinate {
    pub x: CoordinateScalar,
    pub y: CoordinateScalar,
    pub z: CoordinateScalar,
}

impl Coordinate {
    pub fn zero() -> Coordinate {
        Coordinate { x: 0, y: 0, z: 0 }
    }
    #[allow(dead_code)]
    pub fn up() -> Coordinate {
        Coordinate { x: 0, y: 1, z: 0 }
    }
    #[allow(dead_code)]
    pub fn down() -> Coordinate {
        Coordinate { x: 0, y: -1, z: 0 }
    }
    #[allow(dead_code)]
    pub fn left() -> Coordinate {
        Coordinate { x: -1, y: 0, z: 0 }
    }
    #[allow(dead_code)]
    pub fn right() -> Coordinate {
        Coordinate { x: 1, y: 0, z: 0 }
    }
    #[allow(dead_code)]
    pub fn forward() -> Coordinate {
        Coordinate { x: 0, y: 0, z: 1 }
    }
    #[allow(dead_code)]
    pub fn back() -> Coordinate {
        Coordinate { x: 0, y: 0, z: -1 }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Coordinate {}

/// An area in the world.
/// 
/// This is used to represent a volume of space in the world.
pub struct Area {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Area {
    /// Creates a new area with zero volume.
    #[allow(dead_code)]
    pub fn zero() -> Area {
        Area {
            from: Coordinate::zero(),
            to: Coordinate::zero(),
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
    pub fn contains(&self, coord: Coordinate) -> bool {
        coord.x >= self.from.x
            && coord.x <= self.to.x
            && coord.y >= self.from.y
            && coord.y <= self.to.y
            && coord.z >= self.from.z
            && coord.z <= self.to.z
    }

    /// Offsets the area by the given coordinate.
    #[allow(dead_code)]
    pub fn offset(&mut self, coord: Coordinate) {
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