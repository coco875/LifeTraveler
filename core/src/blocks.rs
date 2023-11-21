use block::Block;
use block::SimpleBlock;
use macro_register::register;

#[register(Block)]
pub mod AirBlock {
    use super::*;

    pub static NAME: &str = "Air";

    pub fn new(b: Block) -> SimpleBlock {
        let mut b = SimpleBlock::new(b);
        b.name = NAME;
        b
    }

    pub fn load(b: Block) -> SimpleBlock {
        let mut b = SimpleBlock::new(b);
        b.name = NAME;
        b
    }
}

#[register(Block)]
pub mod StoneBlock {
    use super::*;
}

#[register(Block)]
pub mod DirtBlock {
    use super::*;

    pub fn load(b: Block) -> SimpleBlock {
        let mut b = SimpleBlock::new(b);
        b.name = NAME;
        b
    }
}