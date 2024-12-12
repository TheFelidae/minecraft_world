pub mod backend;

use std::fmt::{self, Debug, Display};
use std::ops::{Add, Sub};

// https://api.luanti.org/map-terminology-and-coordinates/

enum CoordinateFrame {
    Global,
    Local
}

#[derive(Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32
}

impl Coordinate {
    fn zero() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
            z: 0
        }
    }

    fn up() -> Coordinate {
        Coordinate {
            x: 0,
            y: 1,
            z: 0,
        }
    }

    fn down() -> Coordinate {
        Coordinate {
            x: 0,
            y: -1,
            z: 0,
        }
    }

    fn left() -> Coordinate {
        Coordinate {
            x: -1,
            y: 0,
            z: 0,
        }
    }

    fn right() -> Coordinate {
        Coordinate {
            x: 1,
            y: 0,
            z: 0,
        }
    }

    fn forward() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
            z: 1,
        }
    }

    fn back() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
            z: -1,
        }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
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

struct Area {
    from: Coordinate,
    to: Coordinate
}

impl Area {
    fn zero() -> Area {
        Area {
            from: Coordinate::zero(),
            to: Coordinate::zero()
        }
    }

    fn contains(&self, coord: Coordinate) -> bool {
        coord.x >= self.from.x && coord.x <= self.to.x &&
        coord.y >= self.from.y && coord.y <= self.to.y &&
        coord.z >= self.from.z && coord.z <= self.to.z
    }

    fn offset(&mut self, coord: Coordinate) {
        self.from = self.from + coord;
        self.to = self.to + coord;
    }

    /// Returns the contained volume of the area.
    /// 
    /// # Example
    /// ```rust
    /// let area = Area {
    ///    from: Coordinate::zero(),
    ///   to: Coordinate { x: 10, y: 10, z: 10 }
    /// };
    /// 
    /// assert_eq!(area.volume(), 1000);
    /// ```
    fn volume(&self) -> i32 {
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


// A block is the fundamental building block of a World.
trait Block: PartialEq + Eq {
    fn id(&self) -> i32;
    fn name(&self) -> String;
    fn description(&self) -> Option<String>;
}

/// A World Partition is a section of the world that can be loaded and unloaded as needed.
/// These may contain blocks, or may contain further partitions.
trait WorldPartition<T, B: Block> {
    fn area(&self, frame: CoordinateFrame) -> Area;
    fn world_dimensions(&self) -> Coordinate;
    fn local_dimensions(&self) -> Coordinate;
    fn block_at_pos(&self, coord: Coordinate, reference: CoordinateFrame) -> Result<&B, ()>;
    fn block_at_pos_mut(&mut self, coord: Coordinate, reference: CoordinateFrame) -> Result<&mut B, ()>;
    fn child_at_pos(&self, coord: Coordinate) -> Result<&T, ()>;
    fn child_at_pos_mut(&mut self, coord: Coordinate) -> Result<&mut T, ()>;

    fn blocks(&self) -> dyn Iterator<Item=&B>;
    fn blocks_mut(&mut self) -> dyn Iterator<Item=&mut B>;
    fn children(&self) -> dyn Iterator<Item=&T>;
    fn children_mut(&mut self) -> dyn Iterator<Item=&mut T>;
}

/// A World is a collection of blocks - either directly, or through partitions.
trait WorldReader<B: Block, P> {
    fn name(&self) -> String;
    fn description(&self) -> Option<String>;
    
    fn extent_from(&self) -> Coordinate;
    fn extent_to(&self) -> Coordinate;

    fn bottom(&self) -> i32;
    fn top(&self) -> i32;

    fn node_at_pos(&self, coord: Coordinate) -> Result<&B, ()>;
    fn partition_at_pos(&self, coord: Coordinate) -> Result<&P, ()>;

    fn volume(&self) -> i64;

    fn partitions(&self) -> dyn Iterator<Item=&P>;
}

trait WorldWriter<B: Block, P> {
    fn set_node_at_pos(&mut self, coord: Coordinate, block: B) -> Result<(), ()>;
    fn add_partition(&mut self, partition: P) -> Result<(), ()>;
    fn remove_partition(&mut self, partition: P) -> Result<(), ()>;
}