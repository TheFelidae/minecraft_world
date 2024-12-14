// Luanti map file reader/writer

/* -------------------------------------------------------------------------- */
/*                                   Traits                                   */
/* -------------------------------------------------------------------------- */

use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::{Add, Sub},
};

use rusqlite::{params, Connection};

use crate::{Coordinate, CoordinateError, SpatialCoordinate, WorldError};

trait MapReader {
    /// Gets the block at the given coordinate
    ///
    /// # Arguments
    /// - `coord` - The coordinate to get the block at
    ///
    /// # Returns
    /// - The block data, or an error if the block is not found
    fn get_block(&self, coord: HashedCoordinate) -> Result<Vec<u8>, WorldError>;

    /// Checks if a block exists at the given coordinate
    ///
    /// # Arguments
    /// - `coord` - The coordinate to check
    ///
    /// # Returns
    /// - True if the block exists, false otherwise
    fn block_exists(&self, coord: HashedCoordinate) -> Result<bool, WorldError>;

    /// Gets all blocks in the world
    ///
    /// # Returns
    /// - An iterator over all blocks in the world
    fn blocks(&self) -> Result<Vec<HashedCoordinate>, WorldError>;
}

trait MapWriter {
    /// Sets the block at the given coordinate to contain the given data
    ///
    /// # Arguments
    /// - `coord` - The coordinate to set the block at
    /// - `data` - The data to set the block to
    fn set_block(&self, coord: HashedCoordinate, data: &Vec<u8>) -> Result<(), WorldError>;

    /// Removes the block at the given coordinate
    ///
    /// # Arguments
    /// - `coord` - The coordinate to remove the block at
    fn remove_block(&self, coord: HashedCoordinate) -> Result<(), WorldError>;
}

/* -------------------------------------------------------------------------- */
/*                                   Helpers                                  */
/* -------------------------------------------------------------------------- */

/// To store world data efficiently, Luanti uses a *SINGLE* i64 to represent a 3D coordinate.
/// While this allows it to be stored and queried quickly, it unfortunately limits the world size to
/// ~65536x65536x65536 blocks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct HashedCoordinate {
    pub value: i64,
}

impl HashedCoordinate {
    const LIMIT_MIN: i16 = -30920i16;
    const LIMIT_MAX: i16 = 30920i16;
    const LIMIT_MIN_64: i64 = -30920i64;
    const LIMIT_MAX_64: i64 = 30920i64;

    pub fn at(x: i16, y: i16, z: i16) -> Result<HashedCoordinate, CoordinateError> {
        // AABB check for in bounds
        if x < Self::LIMIT_MIN
            || x > Self::LIMIT_MAX
            || y < Self::LIMIT_MIN
            || y > Self::LIMIT_MAX
            || z < Self::LIMIT_MIN
            || z > Self::LIMIT_MAX
        {
            return Err(CoordinateError::OutOfBounds);
        }

        Ok(HashedCoordinate {
            value: i64::from(x) * 16777216i64 + i64::from(y) * 4096i64 + i64::from(z),
        })
    }
}

impl Coordinate for HashedCoordinate {
    type Scalar = i16;
    type Internal = i64;

    fn x(&self) -> Self::Scalar {
        return i16::try_from(self.value / 16777216i64).unwrap();
    }

    fn y(&self) -> Self::Scalar {
        return i16::try_from(self.value / 4096i64).unwrap();
    }

    fn z(&self) -> Self::Scalar {
        return i16::try_from(self.value).unwrap();
    }

    fn zero() -> Self {
        HashedCoordinate { value: 0 }
    }

    fn up() -> Self {
        HashedCoordinate { value: 4096 }
    }

    fn down() -> Self {
        HashedCoordinate { value: -4096 }
    }

    fn left() -> Self {
        HashedCoordinate { value: -1 }
    }

    fn right() -> Self {
        HashedCoordinate { value: 1 }
    }

    fn forward() -> Self {
        HashedCoordinate { value: 16777216 }
    }

    fn back() -> Self {
        HashedCoordinate { value: -16777216 }
    }

    /// Converts a coordinate to a HashedCoordinate.
    ///
    /// # Arguments
    /// - `coord` - The 3D coordinate to convert
    ///
    /// # Returns
    /// - The hashed coordinate
    ///
    /// # Errors
    /// - `WorldError::OutOfBounds` - If the coordinate is outside the bounds of the world
    fn from<T: Coordinate>(coord: T) -> Result<Self, CoordinateError>
    where
        Self::Internal: From<T::Scalar>,
    {
        let from_x = i64::from(coord.x());
        let from_y = i64::from(coord.y());
        let from_z = i64::from(coord.z());
        // AABB check for in bounds (-65535 to 65535 in all directions)
        if from_x < Self::LIMIT_MIN_64
            || from_x > Self::LIMIT_MAX_64
            || from_y < Self::LIMIT_MIN_64
            || from_y > Self::LIMIT_MAX_64
            || from_z < Self::LIMIT_MIN_64
            || from_z > Self::LIMIT_MAX_64
        {
            return Err(CoordinateError::OutOfBounds);
        }

        let hashed = from_x * 16777216i64 + from_y * 4096i64 + from_z;

        Ok(HashedCoordinate { value: hashed })
    }
}

impl Add for HashedCoordinate {
    type Output = HashedCoordinate;

    fn add(self, other: HashedCoordinate) -> HashedCoordinate {
        HashedCoordinate {
            value: self.value.wrapping_add(other.value),
        }
    }
}

impl Sub for HashedCoordinate {
    type Output = HashedCoordinate;

    fn sub(self, other: HashedCoordinate) -> HashedCoordinate {
        HashedCoordinate {
            value: self.value.wrapping_sub(other.value),
        }
    }
}

impl Display for HashedCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HashedCoordinate({}, {}, {})",
            self.x(),
            self.y(),
            self.z()
        )
    }
}

/* -------------------------------------------------------------------------- */
/*                               SQLite3 Backend                              */
/* -------------------------------------------------------------------------- */

/* --------------------------- SQLite3 map reader --------------------------- */

/// A map reader for SQLite3 databases
///
/// This struct is responsible for managing the SQLite3 database file, and querying it for block data.
///
/// Expected schema: `CREATE TABLE `blocks` (`pos` INT NOT NULL PRIMARY KEY, `data` BLOB);`
struct SQLite3MapReader {
    db: Connection,
}

impl SQLite3MapReader {
    fn open_file(file_path: &str) -> Result<SQLite3MapReader, WorldError> {
        let db = Connection::open(file_path).map_err(|_| {
            WorldError::FileNotFound(
                "Failed to open SQLite3 database file: ".to_string() + file_path,
            )
        })?;
        Ok(SQLite3MapReader { db })
    }

    fn open_memory() -> Result<SQLite3MapReader, WorldError> {
        let db = Connection::open_in_memory().map_err(|_| {
            WorldError::FileNotFound("Failed to open SQLite3 database in memory".to_string())
        })?;

        // Create schema
        db.execute(
            "CREATE TABLE `blocks` (`pos` INT NOT NULL PRIMARY KEY, `data` BLOB);",
            params![],
        )
        .map_err(|_| WorldError::DatabaseError("Failed to create blocks table".to_string()))?;

        Ok(SQLite3MapReader { db })
    }
}

impl MapReader for SQLite3MapReader {
    fn block_exists(&self, coord: HashedCoordinate) -> Result<bool, WorldError> {
        // Query block at position
        let mut stmt = self
            .db
            .prepare("SELECT COUNT(*) FROM blocks WHERE pos = ?")
            .map_err(|_| WorldError::DatabaseError("Failed to prepare statement".to_string()))?;
        let count: i64 = stmt
            .query_row(params![coord.value], |row| Ok(row.get(0)?))
            .map_err(|_| WorldError::DatabaseError("Failed to count blocks".to_string()))?;
        Ok(count > 0)
    }

    fn blocks(&self) -> Result<Vec<HashedCoordinate>, WorldError> {
        let mut stmt = self
            .db
            .prepare("SELECT pos FROM blocks")
            .map_err(|_| WorldError::DatabaseError("Failed to prepare statement".to_string()))?;
        let mut rows = stmt
            .query(params![])
            .map_err(|_| WorldError::DatabaseError("Failed to query blocks".to_string()))?;
        let mut coords = Vec::new();
        while let Some(row) = rows
            .next()
            .map_err(|_| WorldError::DatabaseError("Failed to get next row".to_string()))?
        {
            let coord: i64 = row
                .get(0)
                .map_err(|_| WorldError::DatabaseError("Failed to get coordinate".to_string()))?;
            coords.push(HashedCoordinate { value: coord });
        }
        Ok(coords)
    }

    fn get_block(&self, coord: HashedCoordinate) -> Result<Vec<u8>, WorldError> {
        // Query block at position
        let mut stmt = self
            .db
            .prepare("SELECT data FROM blocks WHERE pos = ?")
            .map_err(|_| WorldError::DatabaseError("Failed to prepare statement".to_string()))?;
        struct DbBlockData {
            data: Vec<u8>,
        }

        let block_data: Option<DbBlockData> = stmt
            .query_row(params![coord.value], |row| {
                Ok(DbBlockData { data: row.get(0)? })
            })
            .ok();
        match block_data {
            Some(data) => Ok(data.data),
            None => Err(WorldError::PartitionNotFound(
                <SpatialCoordinate as Coordinate>::from(coord).unwrap(),
            )),
        }
    }
}

impl MapWriter for SQLite3MapReader {
    fn set_block(&self, coord: HashedCoordinate, data: &Vec<u8>) -> Result<(), WorldError> {
        // Query block at position
        let mut stmt = self
            .db
            .prepare("INSERT INTO blocks (pos, data) VALUES (?, ?)")
            .map_err(|_| WorldError::DatabaseError("Failed to prepare statement".to_string()))?;
        stmt.execute(params![coord.value, data])
            .map_err(|_| WorldError::DatabaseError("Failed to insert block".to_string()))?;
        Ok(())
    }

    fn remove_block(&self, coord: HashedCoordinate) -> Result<(), WorldError> {
        // Query block at position
        let mut stmt = self
            .db
            .prepare("DELETE FROM blocks WHERE pos = ?")
            .map_err(|_| WorldError::DatabaseError("Failed to prepare statement".to_string()))?;
        stmt.execute(params![coord.value])
            .map_err(|_| WorldError::DatabaseError("Failed to delete block".to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod luanti_map_sqlite_manager {
    use super::*;

    #[test]
    fn open_simple() {
        let manager = super::SQLite3MapReader::open_memory().unwrap();
    }

    #[test]
    fn insert_block() {
        let manager = super::SQLite3MapReader::open_memory().unwrap();
        let coord = HashedCoordinate::at(0, 0, 0).unwrap();
        let data = vec![0, 1, 2, 3];
        manager.set_block(coord, &data).unwrap();

        let block = manager.get_block(coord).unwrap();
        assert!(block == data);
    }

    #[test]
    fn remove_block() {
        let manager = super::SQLite3MapReader::open_memory().unwrap();
        let coord = HashedCoordinate::at(0, 0, 0).unwrap();
        let data = vec![0, 1, 2, 3];
        manager.set_block(coord, &data).unwrap();

        manager.remove_block(coord).unwrap();
        let block = manager.get_block(coord);
        assert!(block.is_err());
    }

    #[test]
    fn block_exists() {
        let manager = super::SQLite3MapReader::open_memory().unwrap();
        let coord = HashedCoordinate::at(0, 0, 0).unwrap();
        assert!(!manager.block_exists(coord).unwrap());
        let data = vec![0, 1, 2, 3];
        manager.set_block(coord, &data).unwrap();

        let exists = manager.block_exists(coord).unwrap();
        assert!(exists);

        manager.remove_block(coord).unwrap();
        let exists = manager.block_exists(coord).unwrap();
        assert!(!exists);
    }

    #[test]
    fn blocks() {
        let manager = super::SQLite3MapReader::open_memory().unwrap();
        let coord = HashedCoordinate::at(0, 0, 0).unwrap();
        let data = vec![0, 1, 2, 3];
        manager.set_block(coord, &data).unwrap();

        let coords = manager.blocks().unwrap();
        assert!(coords.len() == 1);
        assert!(coords[0] == coord);
    }

    #[test]
    fn out_of_bounds() {
        let coord = HashedCoordinate::at(32500, 0, 0);
        assert!(coord.is_err());
    }
}
