use block::Block;
use core::register::load_block;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block() {
        let block = Block::new();
        assert_eq!(block.id, 0);
    }

    #[test]
    fn test_load_block() {
        let block = Block::new();
        let block = load_block(block);
        assert_eq!(block.name, "Air");
    }

    #[test]
    fn test_load_dirt_block() {
        let mut block = Block::new();
        block.id = 2;
        let block = load_block(block);
        assert_eq!(block.name, "dirt");
    }
}
