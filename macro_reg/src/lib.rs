use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn register(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut text = String::from("#[allow(non_snake_case)]\n");
    text = text + &item.to_string();
    text.parse().unwrap()
}
