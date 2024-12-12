use std::{iter::Map, sync::Arc};

use crate::{
    Area, Block, Coordinate, CoordinateScalar, WorldError, 
    WorldRegistry, WorldPartition, WorldReader, WorldWriter
};

struct MemoryBlock {
    id: Arc<String>,
}

impl Block for MemoryBlock {
    fn id(&self) -> &str {
        self.id.as_str()
    }
}

impl PartialEq for MemoryBlock {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MemoryBlock {}

pub struct MemoryWorldRegistry {
    registry: Map<i32, (String, String)>
}

impl MemoryWorldRegistry {
    pub fn new() -> MemoryWorldRegistry {
        MemoryWorldRegistry {
            registry: Map 
        }
    }

    fn clear(&mut self) {
        self.registry.clear();
    }

    fn insert(&mut self, id: i32, name: String, description: String) {
        self.registry.insert(id, (name, description));
    }

    fn remove(&mut self, id: i32) {
        self.registry.remove(id);
    }

    fn block_name(&self, id: &str) -> Option<String> {
        self.registry.get(id).map(|(name, _)| name.clone())
    }

    fn block_description(&self, id: &str) -> Option<String> {
        self.registry.get(id).map(|(_, description)| description.clone())
    }
}

impl WorldRegistry<MemoryBlock> for MemoryWorldRegistry {
    fn create_block(&self, id: i32) -> Result<MemoryBlock, WorldError> {
        match self.registry.get(id) {
            Some((name, description)) => Ok(MemoryBlock { id, registry: self }),
            None => Err(WorldError::BlockNotFound)
        }
    }

    fn all_blocks(&self) -> Vec<MemoryBlock> {
        todo!()
    }
}

struct MemoryWorldPartition {

}

impl WorldPartition for MemoryWorldPartition {
    fn area(&self, frame: crate::CoordinateFrame) -> Area {
        todo!()
    }

    fn world_dimensions(&self) -> Coordinate {
        todo!()
    }

    fn local_dimensions(&self) -> Coordinate {
        todo!()
    }

    fn block_at_pos(&self, coord: Coordinate, reference: crate::CoordinateFrame) -> Result<&B, ()> {
        todo!()
    }

    fn block_at_pos_mut(
        &mut self,
        coord: Coordinate,
        reference: crate::CoordinateFrame,
    ) -> Result<&mut B, ()> {
        todo!()
    }

    fn child_at_pos(&self, coord: Coordinate) -> Result<&T, ()> {
        todo!()
    }

    fn child_at_pos_mut(&mut self, coord: Coordinate) -> Result<&mut T, ()> {
        todo!()
    }

    fn blocks(&self) -> dyn Iterator<Item = &B> {
        todo!()
    }

    fn blocks_mut(&mut self) -> dyn Iterator<Item = &mut B> {
        todo!()
    }

    fn children(&self) -> dyn Iterator<Item = &T> {
        todo!()
    }

    fn children_mut(&mut self) -> dyn Iterator<Item = &mut T> {
        todo!()
    }
}

struct MemoryWorld {

}

impl WorldReader<MemoryBlock, MemoryWorldPartition> for MemoryWorld {
    fn name(&self) -> String {
        "Generic In-Memory World Data".to_string()
    }

    fn description(&self) -> Option<String> {
        Some("A world stored in memory. Can be reconfigured as needed to suit various world structures.".to_string())
    }

    fn max_area(&self) -> crate::Area {
        Area {
            from: Coordinate {
                x: CoordinateScalar::MIN,
                y: CoordinateScalar::MIN,
                z: CoordinateScalar::MIN,
            },
            to: Coordinate {
                x: CoordinateScalar::MAX,
                y: CoordinateScalar::MAX,
                z: CoordinateScalar::MAX,
            },
        }
    }

    fn bottom(&self) -> CoordinateScalar {
        CoordinateScalar::MIN
    }

    fn top(&self) -> CoordinateScalar {
        CoordinateScalar::MAX
    }

    fn node_at_pos(&self, coord: crate::Coordinate) -> Result<&B, ()> {
        todo!()
    }

    fn partition_at_pos(&self, coord: crate::Coordinate) -> Result<&P, ()> {
        todo!()
    }

    fn volume(&self) -> i64 {
        todo!()
    }

    fn partitions(&self) -> dyn Iterator<Item = &P> {
        todo!()
    }
}