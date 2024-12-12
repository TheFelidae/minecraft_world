use crate::types::{Coordinate, CoordinateScalar, CoordinateFrame, Area};

pub enum WorldError {
    IdNotFound(i32),
    NameNotFound(String),
    OutOfBounds(Coordinate),
    PartitionNotFound(Coordinate),
    CorruptData(String),
    UnknownError(String),
}

// A block is the fundamental building block of a World.
pub trait Block: PartialEq + Eq {
    fn id(&self) -> &str;
    // TODO: Abstracted properties for blocks (This could be sourced from NBT or similar)
}

/// A World Partition is a section of the world that can be loaded and unloaded as needed.
/// These may contain blocks, or may contain further partitions.
pub trait WorldPartition<T, B: Block> {
    fn area(&self, frame: CoordinateFrame) -> Area;
    fn world_dimensions(&self) -> Coordinate;
    fn local_dimensions(&self) -> Coordinate;
    fn block_at_pos(&self, coord: Coordinate, reference: CoordinateFrame) -> Result<&B, ()>;
    fn block_at_pos_mut(
        &mut self,
        coord: Coordinate,
        reference: CoordinateFrame,
    ) -> Result<&mut B, ()>;
    fn child_at_pos(&self, coord: Coordinate) -> Result<&T, ()>;
    fn child_at_pos_mut(&mut self, coord: Coordinate) -> Result<&mut T, ()>;

    fn blocks(&self) -> dyn Iterator<Item = &B>;
    fn blocks_mut(&mut self) -> dyn Iterator<Item = &mut B>;
    fn children(&self) -> dyn Iterator<Item = &T>;
    fn children_mut(&mut self) -> dyn Iterator<Item = &mut T>;
}

/// A World is a collection of blocks - either directly, or through partitions.
pub trait WorldReader<B: Block, P> {
    fn name(&self) -> String;
    fn description(&self) -> Option<String>;

    fn max_area(&self) -> Area;

    /// Returns the lowest possible z-coordinate of the world.
    fn bottom(&self) -> CoordinateScalar;

    /// Returns the highest possible z-coordinate of the world.
    fn top(&self) -> CoordinateScalar;

    fn node_at_pos(&self, coord: Coordinate) -> Result<&B, ()>;
    fn partition_at_pos(&self, coord: Coordinate) -> Result<&P, ()>;

    fn volume(&self) -> i64;

    fn partitions(&self) -> dyn Iterator<Item = &P>;
    fn new_block(&self, id: i32) -> Result<B, WorldError>;
}

pub trait WorldWriter<B: Block, P> {
    fn set_node_at_pos(&mut self, coord: Coordinate, block: B) -> Result<(), WorldError>;
    fn remove_partition(&mut self, coord: Coordinate, frame: CoordinateFrame) -> Result<(), WorldError>;
    fn add_partition(&mut self, partition: P, frame: CoordinateFrame) -> Result<(), WorldError>;
}
