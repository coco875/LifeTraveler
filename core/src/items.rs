//! Items are the basic building blocks of the game. They are used to craft other items, and are the main way of interacting with the world.

use item::SimpleItem;
use macro_register::{add_lang, add_tag, add_tag_from_file, register, register_complement};

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
add_lang!(En, "dirt", "Dirt");

#[register(Item)]
pub mod IronIngotItem {
    use super::*;
    pub static COLOR: &str = "#eeeeee";
}

add_tag!(IronIngotItem, "material", "iron");
add_tag!(IronIngotItem, "materialtype", "ingot");

#[register_complement(Item, "material")]
pub mod ValueOreItem {
    use super::*;
    pub static NAME: &str = "value_ore";
}

add_tag!(ValueOreItem, "materialtype", "ore");
