use block::Block;
use block::SimpleBlock;
use macro_register::register;

#[register(Block)]
pub mod AirBlock {
    use super::*;
    pub fn load(b: Block) -> SimpleBlock {
        let mut b = SimpleBlock::new(b);
        b.name = String::from("Air");
        b
    }
}

#[register(Block)]
pub mod StoneBlock {
    use super::*;
    pub fn load(b: Block) -> SimpleBlock {
        let mut b = SimpleBlock::new(b);
        b.name = String::from("Stone");
        b
    }
}

#[register(Block)]
pub mod DirtBlock {
    use super::*;
    pub fn load(b: Block) -> SimpleBlock {
        let mut b = SimpleBlock::new(b);
        b.name = String::from("Dirt");
        b
    }
}