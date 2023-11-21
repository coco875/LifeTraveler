pub mod blocks;
use block::Block;
use block::SimpleBlock;
pub mod items;
use item::Item;
use item::SimpleItem;

pub fn load_block(block: Block) -> SimpleBlock {
    blocks::REGISTER_BLOCK.get(block.id as usize).unwrap()(block)
}

pub fn create_block(id: i32) -> SimpleBlock {
    load_block(Block { data: 0, id: id as u16 })
}

pub fn load_item(item: Item) -> SimpleItem {
    items::REGISTER_ITEM.get(item.id as usize).unwrap()(item)
}

pub fn create_item(id: i32) -> SimpleItem {
    load_item(Item { data: 0, id: id as u16 })
}