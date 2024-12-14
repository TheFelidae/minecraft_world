use minecraft_world::backend::luanti::world::World;

#[test]
fn basic_open() {
    let world = World::open(std::path::Path::new("assets/world_luanti_5.10")).unwrap();
    assert_eq!(world.game_id(), "minetest");
    assert_eq!(world.damage_enabled(), true);
    assert_eq!(world.creative(), true);
    assert_eq!(world.announcing(), false);
    // Check that mod "worldedit_gui" is false
    assert_eq!(world.mods().contains(&"worldedit_gui".to_string()), false);
}
