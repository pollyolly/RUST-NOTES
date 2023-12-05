use proc_macro::TokenStream;

/** Function Macro **/

#[proc_macro]
pub fn subtract(_input: TokenStream) -> TokenStream {
    "fn diff() -> u32 { 42 - 8 }".parse().unwrap()
}

/** Derive Macro **/

#[proc_macro_derive(Addition)]
pub fn add(_input: TokenStream) -> TokenStream {
    "fn sum() -> u32 { 30 + 5 }".parse().unwrap()
}

/** Attribute Macro **/

#[proc_macro_attribute]
pub fn say_number(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}
