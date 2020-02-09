extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Visibility, VisPublic, token::Pub, Ident, Item};
use quote::quote;

#[proc_macro_attribute]
pub fn pub_if(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    let key = parse_macro_input!(attr as Ident);
    let mut new_input = input.clone();
    let my_pub = Visibility::Public(VisPublic { pub_token: Pub::default() });
    match &mut new_input {
        Item::Const(ref mut x) => x.vis = my_pub,
        Item::Enum(ref mut x) => x.vis = my_pub,
        Item::ExternCrate(ref mut x) => x.vis = my_pub,
        Item::Fn(ref mut x) => x.vis = my_pub,
        Item::Mod(ref mut x) => x.vis = my_pub,
        Item::Static(ref mut x) => x.vis = my_pub,
        Item::Struct(ref mut x) => x.vis = my_pub,
        Item::Trait(ref mut x) => x.vis = my_pub,
        Item::TraitAlias(ref mut x) => x.vis = my_pub,
        Item::Type(ref mut x) => x.vis = my_pub,
        Item::Use(ref mut x) => x.vis = my_pub,
        x => panic!("Unsupported item type: {:?}", quote! { #x }),
    }
    TokenStream::from(quote! {
        #[cfg(#key)]
        #new_input
        #[cfg(not(#key))]
        #input
    })
}
