use super::file_format::KeyValue;

// Based off of the format specified at
// https://github.com/minetest/minetest/blob/master/doc/world_format.md

#[derive(Debug, Copy, Clone)]
pub enum BackendType {
    SQLite3,
    LevelDB,
    Redis,
    PostgreSQL,
    Files, // Files - Deprecated
}

pub struct World {
    // Metadata
    game_id: String,
    enable_damage: bool,
    enable_creative: bool,
    backend: BackendType,
    player_backend: BackendType,
    auth_backend: BackendType,
    mod_storage_backend: BackendType,
    mods: Vec<String>,
    server_announce: bool
}

impl World {
    pub fn open(world_directory: &std::path::Path) -> Result<World, ()> {
        let mut world = World {
            game_id: String::new(),
            enable_damage: false,
            enable_creative: false,
            backend: BackendType::Files,
            player_backend: BackendType::Files,
            auth_backend: BackendType::Files,
            mod_storage_backend: BackendType::Files,
            mods: Vec::new(),
            server_announce: false
        };

        /* -------------------------------------------------------------------------- */
        /*                              Process Metadata                              */
        /* -------------------------------------------------------------------------- */

        // Check for file world.mt existing
        let file_world_mt = world_directory.join("world.mt");
        if !file_world_mt.exists() {
            return Err(());
        }

        // Parse world.mt
        let world_metadata: KeyValue = KeyValue::from(
            std::str::from_utf8(
                std::fs::read(file_world_mt).unwrap().as_slice()
            ).unwrap()
        );
        
        // Fill in the world metadata
        world.game_id = world_metadata.get("gameid").unwrap_or("minetest_game".to_string());
        world.enable_damage = world_metadata.get("enable_damage").unwrap_or("true".to_string()) == "true";
        world.enable_creative = world_metadata.get("creative_mode").unwrap_or("false".to_string()) == "true";
        world.server_announce = world_metadata.get("server_announce").unwrap_or("false".to_string()) == "true";
        
        let load_mods_mt: Vec<String> = world_metadata
            .clone()
            .filter_map(|(key, value)| {
                if key.starts_with("load_mod_") {
                    Some(value)
                } else {
                    None
                }
            })
            .collect();
        world.mods = load_mods_mt;

        world.backend = match world_metadata.get("backend").unwrap_or("files".to_string()).as_str() {
            "sqlite3" => BackendType::SQLite3,
            "leveldb" => BackendType::LevelDB,
            "redis" => BackendType::Redis,
            "postgresql" => BackendType::PostgreSQL,
            _ => BackendType::Files
        };
        world.player_backend = match world_metadata.get("player_backend").unwrap_or("files".to_string()).as_str() {
            "sqlite3" => BackendType::SQLite3,
            "leveldb" => BackendType::LevelDB,
            "redis" => BackendType::Redis,
            "postgresql" => BackendType::PostgreSQL,
            _ => BackendType::Files
        };
        world.auth_backend = match world_metadata.get("auth_backend").unwrap_or("files".to_string()).as_str() {
            "sqlite3" => BackendType::SQLite3,
            "leveldb" => BackendType::LevelDB,
            "redis" => BackendType::Redis,
            "postgresql" => BackendType::PostgreSQL,
            _ => BackendType::Files
        };
        world.mod_storage_backend = match world_metadata.get("mod_storage_backend").unwrap_or("files".to_string()).as_str() {
            "sqlite3" => BackendType::SQLite3,
            "leveldb" => BackendType::LevelDB,
            "redis" => BackendType::Redis,
            "postgresql" => BackendType::PostgreSQL,
            _ => BackendType::Files
        };
        

        Ok(world)
    }
    
    /* ----------------------- Property Getters - Metadata ---------------------- */
    pub fn game_id(&self) -> &str {
        &self.game_id
    }

    pub fn damage_enabled(&self) -> bool {
        self.enable_damage
    }

    pub fn creative(&self) -> bool {
        self.enable_creative
    }

    pub fn announcing(&self) -> bool {
        self.server_announce
    }

    #[allow(dead_code)]
    pub fn backend(&self) -> BackendType {
        self.backend
    }

    #[allow(dead_code)]
    fn player_backend(&self) -> BackendType {
        self.player_backend
    }

    #[allow(dead_code)]
    fn auth_backend(&self) -> BackendType {
        self.auth_backend
    }

    #[allow(dead_code)]
    fn mod_storage_backend(&self) -> BackendType {
        self.mod_storage_backend
    }

    pub fn mods(&self) -> &Vec<String> {
        &self.mods
    }
}