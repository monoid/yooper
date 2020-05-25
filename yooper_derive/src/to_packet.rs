use crate::ast::{
    parse_header_struct, parse_variants, MessageStruct, MessageVariant, VariantMember,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Result};

struct ToPacket<'a>(&'a MessageVariant);

impl MessageVariant {
    fn to_message(&self) -> ToPacket {
        ToPacket(&self)
    }
}

impl<'a> ToTokens for ToPacket<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let MessageVariant {
            name,
            parent,
            reqline,
            nts,
            ..
        } = &self.0;

        let nts_header = if let Some(nts) = nts {
            quote! {
                headers.insert("nts".to_string(), #nts.to_string());
            }
        } else {
            quote! {}
        };

        tokens.extend(quote! {
            #parent::#name ( field ) => {
                let mut headers = field.to_headers();
                #nts_header
                crate::Packet {
                    typ: crate::PacketType::#reqline,
                    headers,
                }
            }
        });
    }
}

impl VariantMember {
    fn to_message(&self) -> ToPacketField {
        ToPacketField(&self)
    }
}

struct ToPacketField<'a>(&'a VariantMember);

impl<'a> ToTokens for ToPacketField<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VariantMember {
            optional,
            header,
            ident,
        } = &self.0;
        let t = if *optional {
            quote! {
                if let Some(v) = &self.#ident {
                    headers.insert(#header.to_string(), v.to_string());
                }
            }
        } else {
            quote! {
                headers.insert(#header.to_string(), self.#ident.to_string());
            }
        };
        tokens.extend(t)
    }
}

struct ToHeaders<'a>(&'a MessageStruct);

impl MessageStruct {
    fn to_headers(&self) -> ToHeaders {
        ToHeaders(&self)
    }
}

impl<'a> ToTokens for ToHeaders<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = self.0.fields.iter().map(VariantMember::to_message);

        tokens.extend(quote! {
            let mut headers = std::collections::HashMap::new();
            #(#fields)*
            headers
        })
    }
}

pub fn headers(input: DeriveInput) -> Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let msgstruct = parse_header_struct(input.clone())?;
    let headers = msgstruct.to_headers();

    let name = input.ident;
    let tokens = quote! {
        #[automatically_derived]
        impl #impl_generics crate::ToHeaders for #name #ty_generics #where_clause {
            fn to_headers(&self) -> crate::Headers {
                #headers
            }
        }
    };

    Ok(tokens)
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let variants = parse_variants(input.clone())?; // TODO(EKF)
    let variants: Vec<ToPacket> = variants.iter().map(MessageVariant::to_message).collect();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = input.ident;

    let tokens = quote! {
        #[automatically_derived]
        impl #impl_generics crate::ToPacket for #name #ty_generics #where_clause {
            fn to_packet(&self) -> crate::Packet {
                match self {
                    #(#variants)*,

                }
            }
        }
    };
    Ok(tokens)
}
