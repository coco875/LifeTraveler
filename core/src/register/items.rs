// This file is generated by build.rs
use item::SimpleItem;

pub struct RegisterItem {
    pub new: fn(SimpleItem) -> SimpleItem,
    pub load: fn(SimpleItem) -> SimpleItem,
    pub tags: &'static quickphf::PhfMap<&'static str, &'static str>,
    pub name: &'static str,
}

pub use crate::items::StoneItem;
pub use crate::items::DirtItem;
pub use crate::items::IronIngotItem;
pub const STONE_ITEM_ID: u16 = 0;
pub static STONE_ITEM_TAGS: &quickphf::PhfMap<&'static str, &'static str> = &::quickphf::PhfMap::new(
    4294967296,
    &[0, 0, 0, 0],
    &[("can_collide", "true"), ("can_render", "true")],
    &[0]
);

pub const DIRT_ITEM_ID: u16 = 1;
pub static DIRT_ITEM_TAGS: &quickphf::PhfMap<&'static str, &'static str> = &::quickphf::PhfMap::new(
    4294967296,
    &[0, 0, 0, 0],
    &[("can_collide", "true"), ("can_render", "true")],
    &[0]
);

pub const IRON_INGOT_ITEM_ID: u16 = 2;
pub static IRON_INGOT_ITEM_TAGS: &quickphf::PhfMap<&'static str, &'static str> = &::quickphf::PhfMap::new(
    4294967296,
    &[0, 0, 0, 0],
    &[("materialtype", "ingot"), ("material", "iron")],
    &[1]
);


pub static REGISTER_ITEM: &[RegisterItem] = &[
    RegisterItem {new: StoneItem::new, load: StoneItem::load, tags: STONE_ITEM_TAGS, name: StoneItem::NAME},
    RegisterItem {new: DirtItem::new, load: DirtItem::load, tags: DIRT_ITEM_TAGS, name: DirtItem::NAME},
    RegisterItem {new: IronIngotItem::new, load: IronIngotItem::load, tags: IRON_INGOT_ITEM_TAGS, name: IronIngotItem::NAME},
];

