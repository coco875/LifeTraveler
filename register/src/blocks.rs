// This file is generated by build.rs
use block::Block;
use block::SimpleBlock;

pub use core::blocks::AirBlock;
pub use core::blocks::StoneBlock;
pub use core::blocks::DirtBlock;

pub static REGISTER_BLOCK: &[fn(Block) -> SimpleBlock] = &[
    AirBlock::load,		// 0
    StoneBlock::load,	// 1
    DirtBlock::load,		// 2
];

pub const AIRBLOCK_ID: u16 = 0;
pub const STONEBLOCK_ID: u16 = 1;
pub const DIRTBLOCK_ID: u16 = 2;
