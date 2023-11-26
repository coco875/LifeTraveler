use std::collections::HashMap;
use std::ffi::c_void;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

pub struct Item {
    pub data: u16,
    pub id: u16,
}

pub struct InventoryData {
    pub inventory: Vec<Item>,
    pub input: fn(InventoryData, Item) -> bool,
    pub output: fn(InventoryData) -> Option<Item>,
}

impl Item {
    pub fn new() -> Self {
        Item { data: 0, id: 0 }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

pub struct SimpleItem {
    pub item: Item,
    pub tags: &'static quickphf::PhfMap<&'static str, &'static str>,
    pub var: HashMap<String, *mut c_void, BuildHasherDefault<XxHash64>>,
    pub name: &'static str,
    pub to_block: fn(&mut SimpleItem) -> &Item,
    pub tick: fn(&mut SimpleItem),
}

pub fn empty_fn(_: &mut SimpleItem) {}
pub fn empty_item(s: &mut SimpleItem) -> &Item {
    &s.item
}

impl SimpleItem {
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

    pub fn set_var<T>(&mut self, name: String, value: T) {
        self.var
            .insert(name, Box::into_raw(Box::new(value)) as *mut c_void);
    }

    pub fn get_var<T>(&self, name: String) -> Option<*mut T> {
        let ptr = self.var.get(&name)?;
        Some(*ptr as *mut T)
    }
}
