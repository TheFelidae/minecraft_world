// Luanti MapBlock Serialization Format Version 29

use crate::SpatialCoordinate;

use super::{LightBank, MapBlockData};

pub struct MapBlock29 {
    header_bytes: [u8; 13]
}

impl MapBlock29 {
    pub fn deserialize(data: &[u8]) -> Self {
        MapBlock29 {
            header_bytes: data[0..13].try_into().unwrap()
        }
    }
}

impl MapBlockData for MapBlock29 {
    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    fn underground(&self) -> bool {
        // byte 2, 0x01 flag
        self.header_bytes[2] & 0x01 != 0
    }

    fn day_night_differs(&self) -> bool {
        // byte 2, 0x02 flag
        self.header_bytes[2] & 0x02 != 0
    }

    fn light_dirty(&self) -> bool {
        // byte 2, 0x04 flag
        self.header_bytes[2] & 0x04 != 0
    }

    fn was_generated(&self) -> bool {
        // byte 2, 0x08 flag
        self.header_bytes[2] & 0x08 != 0
    }

    fn light_complete(&self, bank: LightBank, direction: SpatialCoordinate) -> bool {
        false
    }

    fn timestamp(&self) -> u32 {
        0
    }
}