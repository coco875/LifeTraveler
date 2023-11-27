use block::SimpleBlock;
use macro_register::{add_lang, add_lang_from_file, add_tag, register};

#[register(Block)]
pub mod AirBlock {
    use super::*;

    pub static NAME: &str = "Air";

    pub fn new(mut b: SimpleBlock) -> SimpleBlock {
        b
    }

    pub fn load(mut b: SimpleBlock) -> SimpleBlock {
        b
    }
}

add_tag!(AirBlock, "can_collide", "false");
add_tag!(AirBlock, "can_render", "false");

add_lang!(Fr, "Air", "Air");

#[register(Block)]
pub mod StoneBlock {
    use super::*;
}

add_lang!(En, "stone", "Stone");
add_lang!(Fr, "stone", "Pierre");

#[register(Block)]
pub mod DirtBlock {
    use super::*;

    pub fn load(mut b: SimpleBlock) -> SimpleBlock {
        b
    }
}

add_lang_from_file!(En, "common_block_en.json");
