use std::ffi::c_void;
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
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
    pub var: HashMap<String, *mut c_void, BuildHasherDefault<XxHash64>>,
    pub name: String,
    pub tick: fn(&mut SimpleItem),
}

pub fn empty_fn(_: &mut SimpleItem) {}

impl SimpleItem {
    pub fn new(item: Item) -> Self {
        SimpleItem {
            item,
            var: Default::default(),
            name: String::new(),
            tick: empty_fn
        }
    }

    pub fn set_var<T>(&mut self, name: String, value: T) {
        self.var.insert(name, Box::into_raw(Box::new(value)) as *mut c_void);
    }

    pub fn get_var<T>(&self, name: String) -> Option<*mut T> {
        let ptr = self.var.get(&name)?;
        Some(*ptr as *mut T)
    }

}