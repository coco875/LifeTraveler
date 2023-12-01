//! # Block
//! Block is a library to give a base structure for blocks in a game.
//! the [`Block`] struct is the base struct for all blocks and it's what it's stored in the world.
//! the [`SimpleBlock`] struct is a simple implementation of the [`Block`] struct to decompress variables and functions associated with the block.
//! to register the block see [`macro_register`]

use serde_json::Value;
use std::collections::HashMap;
use std::ffi::c_void;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

pub struct Block {
    pub id: u16,
}

impl Block {
    pub fn new() -> Self {
        Block { id: 0 }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

pub struct SimpleBlock {
    pub block: Block,
    pub tags: &'static quickphf::PhfMap<&'static str, &'static str>,
    pub var: HashMap<String, Value, BuildHasherDefault<XxHash64>>,
    pub name: &'static str,
    pub to_block: fn(&mut SimpleBlock) -> &Block,
    pub tick: fn(&mut SimpleBlock),
}

pub fn empty_fn(_: &mut SimpleBlock) {}
pub fn empty_block(s: &mut SimpleBlock) -> &Block {
    &s.block
}

impl SimpleBlock {
    pub fn new(
        block: Block,
        name: &'static str,
        tags: &'static quickphf::PhfMap<&'static str, &'static str>,
    ) -> Self {
        SimpleBlock {
            block,
            tags,
            var: Default::default(),
            name,
            to_block: empty_block,
            tick: empty_fn,
        }
    }

    /// Set a variable/attribute of the item
    pub fn set_var<T>(&mut self, name: String, value: T) {
        self.var.insert(name, serde_json::to_value(value).unwrap());
    }

    /// Get a variable/attribute of the item
    pub fn get_var<T>(&self, name: String) -> &T {
        self.var
            .get(&name)
            .unwrap()
            .as_any()
            .downcast_ref::<T>()
            .unwrap()
    }
}
