pub mod blocks;
use block::Block;
use block::SimpleBlock;
pub mod items;
use item::Item;
use item::SimpleItem;

pub fn load_block(block: Block) -> SimpleBlock {
    blocks::REGISTER_BLOCK.get(block.id as usize).unwrap()(block)
}

pub fn load_item(item: Item) -> SimpleItem {
    items::REGISTER_ITEM.get(item.id as usize).unwrap()(item)
}