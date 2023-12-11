//! # Macro Reg
//! This crate is a macro to generate code for the registry of the game.
//! some are just decoration and others are really useful.

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use regex::Regex;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::Item;
use syn::ItemMod;

/// This macro is used to register a block, item, entity, etc...
/// this macro will automatically generate missing functions and variables.
/// Variables:
/// - NAME: the name of the block, item, entity, etc... (automatically generated by name of the module remove the "entity" or "block" or "item" and convert to snake case)
/// Functions:
/// - load: this function is called when the game is loading the block, item, entity, etc...
/// - new: this function is called when the game is creating the block, item, entity, etc...
///
/// # Example
/// ```rust
/// #[register(Block)]
///pub mod StoneBlock {
///     use super::*;
/// }
/// ```
/// no warning about non snake case because of the `allow_non_snake_case` attribute
/// he have NAME who is "stone" and load and new function who juste return the simple block given in parameter
/// an exemple de load function:
/// ```rust
/// pub fn load(mut b: SimpleBlock) -> SimpleBlock {
///    b
/// }
/// ```
/// return the simple block given in parameter
#[proc_macro_attribute]
pub fn register(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let re = Regex::new(r"pub *\n* *mod *\n* *(\w+)").unwrap();
    let binding = &item.to_string();
    let mut binding = re.captures(binding).unwrap()[1].to_string();

    let kind_register = _attr.to_string();
    let kind_register = kind_register.split(",").collect::<Vec<&str>>()[0];

    binding = binding.replace(kind_register, "");
    let name = &binding.to_case(Case::Snake);

    let mut input = parse_macro_input!(item as ItemMod);

    let name_function = quote::format_ident!("load_{}", kind_register.to_lowercase());

    let load_function = quote! {
        pub use self::core::default_function::#name_function as load;
    };

    let name_function = quote::format_ident!("new_{}", kind_register.to_lowercase());

    let new_function = quote! {
        pub use self::core::default_function::#name_function as new;
    };

    let name_variable = quote! {
        /// automatically generated by macro_reg
        /// this variable is the name of the block, item, entity, etc...
        /// value: #name
        pub static NAME: &str = #name;
    };

    let allow_non_snake_case: syn::Attribute = parse_quote! {
        #[allow(non_snake_case)]
    };

    let allow_non_used_mut: syn::Attribute = parse_quote! {
        #[allow(unused_mut)]
    };
    // let mut text = String::from("#[allow(non_snake_case)]\n");
    // text = text + &item.to_string();
    input.attrs.push(allow_non_snake_case);
    input.attrs.push(allow_non_used_mut);

    let re = Regex::new(r"pub *\n* *fn *\n* *load").unwrap();

    if !re.is_match(&input.to_token_stream().to_string()) {
        input
            .content
            .as_mut()
            .unwrap()
            .1
            .push(Item::Verbatim(load_function));
    }

    let re = Regex::new(r"pub *\n* *fn *\n* *new").unwrap();

    if !re.is_match(&input.to_token_stream().to_string()) {
        input
            .content
            .as_mut()
            .unwrap()
            .1
            .push(Item::Verbatim(new_function));
    }

    let re = Regex::new(r"pub *\n* *static *\n* *NAME").unwrap();

    if !re.is_match(&input.to_token_stream().to_string()) {
        input
            .content
            .as_mut()
            .unwrap()
            .1
            .push(Item::Verbatim(name_variable));
    }

    let code = quote!(
          #input
    );

    TokenStream::from(code)
}

/// This macro is used to register a block, item, entity, etc... who are in link with a tag.
/// this macro will automatically generate missing functions and variables.
/// # Example
/// ```rust
/// #[register_complement(Item, "material")]
/// pub mod ValueOreItem {
///    use super::*;
///   pub static NAME: &str = "value_ore";
/// }
/// ```
/// no warning about non snake case because of the `allow_non_snake_case` attribute
/// for every tag "material" this will generate a new ValueOreItem and replace the "Value" by the tag value in Pascal case and the "value" by the tag value
/// he make this very dummy and replace every "Value" and "value" by the tag value even if it's not in the name of the variable or function
#[proc_macro_attribute]
pub fn register_complement(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // get file at the source of the project
    let path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .to_path_buf();
    let name = _attr.to_string().split(", ").collect::<Vec<&str>>()[1].replace('"', "");
    let file_outpath = Path::new(&path)
        .join("tags_output")
        .join(format!("{}.txt", name));
    // open file
    let mut file = fs::File::open(&file_outpath)
        .unwrap_or_else(|e| panic!("no file found at {:?} with error: {}", file_outpath, e));
    // read file
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let list_values = content.split("\n").collect::<Vec<&str>>();

    let item = register(_attr, item);
    let text = item.to_string();
    let mut result = String::new();
    for value in list_values {
        let mut text = text.clone();
        text = text.replace("Value", &value.to_case(Case::Pascal));
        text = text.replace("value", value);
        result = result + &text + "\n";
    }
    result.parse().unwrap()
}

/// This macro is used to add a tag to a block, item, entity, etc...
/// # Example
/// ```rust
/// #[register(Block)]
/// pub mod StoneBlock {
///    use super::*;
/// }
/// add_tag!(StoneBlock, "can_collide", "true");
/// ```
/// the macro will add any code it's just a indication for build.rs
/// so StoneBlock will have a tag "can_collide" with the value "true"
#[proc_macro]
pub fn add_tag(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

/// This macro is used to add a tag to a block, item, entity, etc... from a file
/// # Example
/// ```rust
/// #[register(Block)]
/// pub mod StoneBlock {
///   use super::*;
/// }
/// add_tag_from_file!(StoneBlock, "stone_block_tags.json");
/// ```
/// the macro will add any code it's just a indication for build.rs
/// if the file is:
/// ```json
/// {
///    "can_collide": "true",
///   "can_render": "true"
/// }
/// ```
/// so StoneBlock will have a tag "can_collide" with the value "true" and a tag "can_render" with the value "true"
#[proc_macro]
pub fn add_tag_from_file(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

/// This macro is used to add a lang to a block, item, entity, etc...
/// # Example
/// ```rust
/// #[register(Block)]
/// pub mod StoneBlock {
///   use super::*;
/// }
/// add_lang!(En, "stone", "Stone");
/// ```
/// the macro will add any code it's just a indication for build.rs
/// so StoneBlock will have for en "stone" with the value "Stone"
#[proc_macro]
pub fn add_lang(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

/// This macro is used to add a lang to a block, item, entity, etc... from a file
/// # Example
/// ```rust
/// #[register(Block)]
/// pub mod StoneBlock {
///  use super::*;
/// }
/// add_lang_from_file!(En, "stone_block_lang.json");
/// ```
/// the macro will add any code it's just a indication for build.rs
/// if the file is:
/// ```json
/// {
///   "stone": "Stone",
///  "stone_block": "Stone Block"
/// }
/// ```
/// so StoneBlock will have for en "stone" with the value "Stone" and "stone_block" with the value "Stone Block" in English
#[proc_macro]
pub fn add_lang_from_file(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}
