use item::SimpleItem;
use macro_register::{
    register, 
    add_tag, 
    add_tag_from_file, 
    register_complement
};

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

#[register(Item)]
pub mod IronIngotItem {
    use super::*;
    pub static COLOR: &str = "#eeeeee";
}

add_tag!(IronIngotItem, "material", "iron");
add_tag!(IronIngotItem, "materialtype", "ingot");

#[register_complement("material")]
pub mod OreItem {
    use super::*;
}