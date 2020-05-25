mod ast;
mod from_message;
mod to_message;

extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromPacket, attributes(header, message))]
pub fn derive_from_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let e = parse_macro_input!(input as DeriveInput);
    from_message::derive(e)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(ToPacket, attributes(header, message))]
pub fn derive_to_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let e = parse_macro_input!(input as DeriveInput);
    to_message::derive(e)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
