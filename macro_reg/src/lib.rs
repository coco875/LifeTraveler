use proc_macro::TokenStream;
use quote::quote;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::ItemMod;
use syn::Item;
use syn::parse_quote;
use regex::Regex;
use convert_case::{Case, Casing};

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
    
    let mut _type = quote::format_ident!("Simple{}", kind_register);

    let load_function = quote! {
        pub fn load(mut b: #_type) -> #_type {
            b
        }
    };

    let new_function = quote! {
        pub fn new(mut b: #_type) -> #_type {
            b
        }
    };

    let name_variable = quote! {
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
        input.content.as_mut().unwrap().1.push(Item::Verbatim(load_function));
    }

    let re = Regex::new(r"pub *\n* *fn *\n* *new").unwrap();

    if !re.is_match(&input.to_token_stream().to_string()) {
        input.content.as_mut().unwrap().1.push(Item::Verbatim(new_function));
    }

    let re = Regex::new(r"pub *\n* *static *\n* *NAME").unwrap();

    if !re.is_match(&input.to_token_stream().to_string()) {
        input.content.as_mut().unwrap().1.push(Item::Verbatim(name_variable));
    }


    let code = quote!(
          #input
    );

    TokenStream::from(code)
}
#[proc_macro_attribute]
pub fn register_complement(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // get file at the source of the project
    let mut path = env::current_dir().unwrap();
    while !path.ends_with("LifeTraveler") {
        path = path.parent().unwrap().to_path_buf();
    }
    println!("path: {}", path.to_str().unwrap());
    let name = _attr.to_string().split(", ").collect::<Vec<&str>>()[1].replace('"', "");
    println!("name: {}", name);
    let file_outpath = Path::new(&path).join("tags_output").join(format!("{}.txt", name));
    println!("file: {}", file_outpath.to_str().unwrap());
    // open file
    let mut file = fs::File::open(&file_outpath).unwrap();
    print!("content: ");
    // read file
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let list_values = content.split("\n").collect::<Vec<&str>>();
    let item = register(_attr, item);
    let text = item.to_string();
    let mut result = String::new();
    for value in list_values {
        println!("{}", value);
        let mut text = text.clone();
        text = text.replace("Value", &value.to_case(Case::Pascal));
        text = text.replace("value", value);
        result = result + &text+"\n";
    }
    println!("{}", result);
    result.parse().unwrap()
}

#[proc_macro]
pub fn add_tag(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

#[proc_macro]
pub fn add_tag_from_file(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}