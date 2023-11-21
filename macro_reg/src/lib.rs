use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::ItemMod;
use syn::Item;
use syn::parse_quote;
use regex::Regex;

#[proc_macro_attribute]
pub fn register(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let re = Regex::new(r"pub *\n* *mod *\n* *(\w+)").unwrap();
    let mut binding = item.to_string();
    binding = re.captures(&binding).unwrap()[1].to_string();
    binding = binding.replace(&_attr.to_string(), "");
    let name = &binding;

    let mut input = parse_macro_input!(item as ItemMod);

    let load_function = quote! {
        pub fn load(mut b: SimpleBlock) -> SimpleBlock {
            b.name = NAME;
            b
        }
    };

    let new_function = quote! {
        pub fn new(mut b: SimpleBlock) -> SimpleBlock {
            b.name = NAME;
            b
        }
    };

    let name_variable = quote! {
        pub static NAME: &str = #name;
    };

    let allow_non_snake_case: syn::Attribute = parse_quote! {
        #[allow(non_snake_case)]
    };
    // let mut text = String::from("#[allow(non_snake_case)]\n");
    // text = text + &item.to_string();
    input.attrs.push(allow_non_snake_case);

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

#[proc_macro]
pub fn add_tag(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}