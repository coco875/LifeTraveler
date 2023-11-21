use block::Block;
use block::SimpleBlock;
use macro_register::{register, add_tag};

#[register(Block)]
pub mod AirBlock {
    use super::*;

    pub static NAME: &str = "Air";

    pub fn new(mut b: SimpleBlock) -> SimpleBlock {
        b.name = NAME;
        b
    }

    pub fn load(mut b: SimpleBlock) -> SimpleBlock {
        b.name = NAME;
        b
    }
}

add_tag!(AirBlock, "can_collide", "false");
add_tag!(AirBlock, "can_render", "false");

#[register(Block)]
pub mod StoneBlock {
    use super::*;
}

#[register(Block)]
pub mod DirtBlock {
    use super::*;

    pub fn load(mut b: SimpleBlock) -> SimpleBlock {
        b.name = NAME;
        b
    }
}