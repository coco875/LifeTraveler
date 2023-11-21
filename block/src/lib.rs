use std::ffi::c_void;
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
use twox_hash::XxHash64;
pub mod block_complement;

pub struct Block {
    pub data: u16,
    pub id: u16,
}

impl Block {
    pub fn new() -> Self {
        Block { data: 0, id: 0 }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }
}

pub struct SimpleBlock {
    pub block: Block,
    pub var: HashMap<String, *mut c_void, BuildHasherDefault<XxHash64>>,
    pub name: &'static str,
    pub to_block: fn(&mut SimpleBlock) -> &Block,
    pub tick: fn(&mut SimpleBlock),
}

pub fn empty_fn(_: &mut SimpleBlock) {}
pub fn empty_block(s: &mut SimpleBlock) -> &Block { &s.block }

impl SimpleBlock {
    pub fn new(block: Block) -> Self {
        SimpleBlock {
            block,
            var: Default::default(),
            name: "",
            to_block: empty_block,
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
