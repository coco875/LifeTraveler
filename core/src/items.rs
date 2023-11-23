use item::SimpleItem;
use macro_register::{register, add_tag, add_tag_from_file};

#[register(Item)]
pub mod StoneItem {
    use super::*;

    pub static NAME: &str = "Air";

    pub fn new(mut b: SimpleItem) -> SimpleItem {
        b
    }

    pub fn load(mut b: SimpleItem) -> SimpleItem {
        b
    }
}

add_tag!(StoneItem, "can_collide", "true");
add_tag!(StoneItem, "can_render", "true");

#[register(Item)]
pub mod DirtItem {
    use super::*;
}

add_tag_from_file!(DirtItem, "dirt_item_tags.json");