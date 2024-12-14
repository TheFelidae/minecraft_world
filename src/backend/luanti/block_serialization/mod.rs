use v29::MapBlock29;

use crate::SpatialCoordinate;
pub mod v29;

pub enum LightBank {
    Day,
    Night
}

pub trait MapBlockData {
    fn serialize(&self) -> Vec<u8>;

    /// False if the block has visibility to the sky
    fn underground(&self) -> bool;
    /// True if this block contains a Node that differs in lighting between day and night
    fn day_night_differs(&self) -> bool;
    /// True if the lighting needs to be recalculated
    fn light_dirty(&self) -> bool;
    fn was_generated(&self) -> bool;
    fn light_complete(&self, bank: LightBank, direction: SpatialCoordinate) -> bool;
    fn timestamp(&self) -> u32;
}

fn deserialize_block_data(data: &Vec<u8>) -> Result<Box<dyn MapBlockData>, ()> {
    match data[0] {
        29 => Ok(Box::new(MapBlock29::deserialize(&data[1..]))),
        _ => Err(())
    }
}