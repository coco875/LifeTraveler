//! Interfaces for registering blocks and items
//! # Example
//! ```rust
//! use core::register::blocks::DIRT_BLOCK_ID;
//! use core::register::create_block;
//!
//! // Create a dirt block
//! let block = create_block(DIRT_BLOCK_ID);
//!
//! // Get the block id
//! assert_eq!(block.get_id(), DIRT_BLOCK_ID);
//! ```
//! ```rust
//! use core::register::items::DIRT_ITEM_ID;
//! use core::register::load_item;
//!
//! // Create a dirt item
//! let item = load_item(core::item::Item { id: DIRT_ITEM_ID });
//!
//! // Get the item id
//! assert_eq!(item.get_id(), DIRT_ITEM_ID);
//! ```

pub mod blocks;
use block::Block;
use block::SimpleBlock;
pub mod items;
use item::Item;
use item::SimpleItem;

/// Load a block from a Block struct
pub fn load_block(block: Block) -> SimpleBlock {
    let r = blocks::REGISTER_BLOCK.get(block.id as usize).unwrap();
    let sb = SimpleBlock::new(block, r.name, r.tags);
    (r.load)(sb)
}

/// Create a block from an id
pub fn create_block(id: u16) -> SimpleBlock {
    let b = Block { id: id };
    let r = blocks::REGISTER_BLOCK.get(id as usize).unwrap();
    let sb = SimpleBlock::new(b, r.name, r.tags);
    (r.new)(sb)
}

/// Load an item from an Item struct
pub fn load_item(item: Item) -> SimpleItem {
    let r = items::REGISTER_ITEM.get(item.id as usize).unwrap();
    let sb = SimpleItem::new(item, r.name, r.tags);
    (r.load)(sb)
}

/// Create an item from an id
pub fn create_item(id: i32) -> SimpleItem {
    let b = Item { id: id as u16 };
    let r = items::REGISTER_ITEM.get(id as usize).unwrap();
    let sb = SimpleItem::new(b, r.name, r.tags);
    (r.new)(sb)
}
