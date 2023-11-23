// This file is generated by build.rs
use item::SimpleItem;

pub struct RegisterItem {
        pub new: fn(SimpleItem) -> SimpleItem,
        pub load: fn(SimpleItem) -> SimpleItem,
        pub tags: &'static quickphf::PhfMap<&'static str, &'static str>,
        pub name: &'static str,
    }

pub use core::items::StoneItem;
pub use core::items::DirtItem;
pub use core::items::IronIngotItem;
pub const STONEITEM_ID: i32 = 0;
pub static STONEITEM_TAGS: &quickphf::PhfMap<&'static str, &'static str> = &::quickphf::PhfMap::new(
    4294967296,
    &[0, 0, 0, 0],
    &[("can_collide", "true"), ("can_render", "true")],
    &[0]
);

pub const DIRTITEM_ID: i32 = 1;
pub static DIRTITEM_TAGS: &quickphf::PhfMap<&'static str, &'static str> = &::quickphf::PhfMap::new(
    4294967296,
    &[0, 0, 0, 0],
    &[("can_collide", "true"), ("can_render", "true")],
    &[0]
);

pub const IRONINGOTITEM_ID: i32 = 2;
pub static IRONINGOTITEM_TAGS: &quickphf::PhfMap<&'static str, &'static str> = &::quickphf::PhfMap::new(
    4294967296,
    &[0],
    &[("material", "ore")],
    &[0, 0]
);


pub static REGISTER_ITEM: &[RegisterItem] = &[
    RegisterItem {new: StoneItem::new, load: StoneItem::load, tags: STONEITEM_TAGS, name: StoneItem::NAME},
    RegisterItem {new: DirtItem::new, load: DirtItem::load, tags: DIRTITEM_TAGS, name: DirtItem::NAME},
    RegisterItem {new: IronIngotItem::new, load: IronIngotItem::load, tags: IRONINGOTITEM_TAGS, name: IronIngotItem::NAME},
];

