//! # Item
//! Items are the basic building blocks of the game. They are used to craft other items, and are the main way of interacting with the world.
//! see [`core::items`] for examples

use serde_json::Value;
use std::collections::HashMap;
use std::ffi::c_void;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

/// The basic struct for an item
pub struct Item {
    pub id: u16,
}

/// The basic struct for an item
impl Item {
    pub fn new() -> Self {
        Item { id: 0 }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

/// The simple struct for an item
pub struct SimpleItem {
    /// The item
    pub item: Item,
    /// The static tags
    pub tags: &'static quickphf::PhfMap<&'static str, &'static str>,
    /// The dynamic tags/attributes
    pub var: HashMap<String, Value, BuildHasherDefault<XxHash64>>,
    /// The name/id of the item
    pub name: &'static str,
    /// The function to convert the simple item to an item
    pub to_block: fn(&mut SimpleItem) -> &Item,
    /// The function to tick the item
    pub tick: fn(&mut SimpleItem),
}

pub fn empty_fn(_: &mut SimpleItem) {}
pub fn empty_item(s: &mut SimpleItem) -> &Item {
    &s.item
}

/// The simple struct for an item
impl SimpleItem {
    /// Create a new simple item
    pub fn new(
        item: Item,
        name: &'static str,
        tags: &'static quickphf::PhfMap<&'static str, &'static str>,
    ) -> Self {
        SimpleItem {
            item,
            tags,
            var: Default::default(),
            name,
            to_block: empty_item,
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
