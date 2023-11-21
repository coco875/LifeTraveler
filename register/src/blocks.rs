// This file is generated by build.rs
use block::SimpleBlock;
use phf::phf_map;

pub struct RegisterBlock {
    pub new: fn(SimpleBlock) -> SimpleBlock,
    pub load: fn(SimpleBlock) -> SimpleBlock,
    pub tags: &'static phf::Map<&'static str, &'static str>,
    pub name: &'static str,
}

pub use core::blocks::AirBlock;
pub use core::blocks::StoneBlock;
pub use core::blocks::DirtBlock;
pub const AIRBLOCK_ID: u16 = 0;
pub static AIRBLOCK_TAGS: &phf::Map<&'static str, &'static str> = &phf_map! {
    "can_collide" => "false",
    "can_render" => "false",
};

pub const STONEBLOCK_ID: u16 = 1;
pub static STONEBLOCK_TAGS: &phf::Map<&'static str, &'static str> = &phf_map! {
};

pub const DIRTBLOCK_ID: u16 = 2;
pub static DIRTBLOCK_TAGS: &phf::Map<&'static str, &'static str> = &phf_map! {
};


pub static REGISTER_BLOCK: &[RegisterBlock] = &[
    RegisterBlock {new: AirBlock::new, load: AirBlock::load, tags: AIRBLOCK_TAGS, name: AirBlock::NAME},
    RegisterBlock {new: StoneBlock::new, load: StoneBlock::load, tags: STONEBLOCK_TAGS, name: StoneBlock::NAME},
    RegisterBlock {new: DirtBlock::new, load: DirtBlock::load, tags: DIRTBLOCK_TAGS, name: DirtBlock::NAME},
];

