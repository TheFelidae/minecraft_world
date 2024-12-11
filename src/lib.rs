// https://api.luanti.org/map-terminology-and-coordinates/

// A node is the fundamental building block of a Luanti world.
trait Node {
    fn get_id(&self) -> i32;
    fn get_name(&self) -> String;
}

// A mapblock (or block) is a 16x16x16 cube of nodes, and the equivalent of a Minecraft chunk.
trait Mapblock {

}

// A mapchunk (or chunk) is a (typically) 5x5x5 cube of mapblocks, and the equivalent of a Minecraft region.
trait Mapchunk {

}

trait World {

}