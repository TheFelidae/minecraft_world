enum BackendType {
    SQLite3,
    LevelDB,
    Redis,
    PostgreSQL,
    Files
    // Files - Deprecated
}

/// Metadata about a World - e.g. loaded mods, settings, etc.
trait Metadata {
    /// The backend used for the world data
    /// 
    /// This is used for world data, such as nodes, entities, etc.
    fn world_backend(&self) -> BackendType;
    /// The backend used for player data
    /// 
    /// This is used for player data, such as inventory, position, etc.
    fn player_backend(&self) -> BackendType;
    fn auth_backend(&self) -> BackendType;
    fn mod_storage_backend(&self) -> BackendType;
    fn announcing(&self) -> bool;
    fn damage_enabled(&self) -> bool;
    fn creative_enabled(&self) -> bool;
    fn game_id(&self) -> String;
    fn mods(&self) -> Vec<String>;
}