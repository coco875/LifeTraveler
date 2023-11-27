pub mod blocks;
use block::Block;
use block::SimpleBlock;
pub mod items;
use item::Item;
use item::SimpleItem;

pub fn load_block(block: Block) -> SimpleBlock {
    let r = blocks::REGISTER_BLOCK.get(block.id as usize).unwrap();
    let sb = SimpleBlock::new(block, r.name, r.tags);
    (r.load)(sb)
}

pub fn create_block(id: i32) -> SimpleBlock {
    let b = Block {
        data: 0,
        id: id as u16,
    };
    let r = blocks::REGISTER_BLOCK.get(id as usize).unwrap();
    let sb = SimpleBlock::new(b, r.name, r.tags);
    (r.new)(sb)
}

pub fn load_item(item: Item) -> SimpleItem {
    let r = items::REGISTER_ITEM.get(item.id as usize).unwrap();
    let sb = SimpleItem::new(item, r.name, r.tags);
    (r.load)(sb)
}

pub fn create_item(id: i32) -> SimpleItem {
    let b = Item {
        data: 0,
        id: id as u16,
    };
    let r = items::REGISTER_ITEM.get(id as usize).unwrap();
    let sb = SimpleItem::new(b, r.name, r.tags);
    (r.new)(sb)
}
