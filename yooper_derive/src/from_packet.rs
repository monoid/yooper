use crate::ast::{
    parse_header_struct, parse_variants, MessageStruct, MessageVariant, VariantMember,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Result};

// #[derive(ToMessage, FromMessage)]
// enum Packet {
//   #[message(reqline = Notify, nts = "ssdp:alive")]
//   Alive (Alive)
//     #[header("ssdpuuid.upnp.org")
//     uuid: String
//   }
//  }
// }
//
// #[deive(ToHeaders, FromHeaders)]
// struct Alive {
//     #[header("ssdpuuid.upnp.org")
//     uuid: String
// }

struct FromPacket<'a>(&'a MessageVariant);

impl MessageVariant {
    fn from_message(&self) -> FromPacket {
        FromPacket(&self)
    }
}

impl<'a> FromPacket<'a> {
    fn as_from_condition(&self) -> TokenStream {
        let mut tokens = TokenStream::new();

        let MessageVariant { reqline, nts, .. } = &self.0;

        tokens.extend(quote! { packet.typ == crate::ssdp::packet::PacketType::#reqline});
        if let Some(nts) = nts {
            tokens.extend(quote! {
                && packet.headers.get("nts").map_or(false, |h| h == #nts )
            })
        }

        tokens
    }
}

impl<'a> ToTokens for FromPacket<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let MessageVariant {
            name,
            parent,
            struct_name,
            ..
        } = &self.0;
        let cond = self.as_from_condition();
        tokens.extend(quote! {
            if #cond {
                return Ok(
                    #parent::#name(#struct_name::from_headers(&packet.headers)?)
                )
            }
        });
    }
}

impl VariantMember {
    fn from_message(&self) -> FromPacketField {
        FromPacketField(&self)
    }
}

struct FromPacketField<'a>(&'a VariantMember);

impl<'a> ToTokens for FromPacketField<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VariantMember {
            optional,
            header,
            ident,
            ..
        } = self.0;

        let q = if *optional {
            quote! {
                #ident: headers.get(#header).map_or(Ok(None), |v| v.parse().map(Some))?
            }
        } else {
            quote! {
                #ident: headers.get(#header).ok_or_else(|| crate::Error::MissingHeader(#header))?.parse()?
            }
        };

        tokens.extend(q);
    }
}

struct FromHeaders<'a>(&'a MessageStruct);

impl MessageStruct {
    fn from_headers(&self) -> FromHeaders {
        FromHeaders(&self)
    }
}

impl<'a> ToTokens for FromHeaders<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = self.0.fields.iter().map(VariantMember::from_message);
        let name = &self.0.name;

        tokens.extend(quote! {
           Ok(#name {
                #(#fields),*
           })
        })
    }
}

pub fn headers(input: DeriveInput) -> Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let msgstruct = parse_header_struct(input.clone())?;
    let headers = msgstruct.from_headers();

    let name = input.ident;

    let tokens = quote! {
        #[automatically_derived]
        impl #impl_generics crate::ssdp::packet::FromHeaders for #name #ty_generics #where_clause {

            fn from_headers(headers: &crate::ssdp::packet::Headers) -> Result<Self, crate::Error> {
                #headers
            }
        }
    };
    Ok(tokens)
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let variants = parse_variants(input.clone())?; // TODO(EKF)
    let variants: Vec<FromPacket> = variants.iter().map(MessageVariant::from_message).collect();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = input.ident;

    let tokens = quote! {
        #[automatically_derived]
        impl #impl_generics crate::ssdp::packet::FromPacket for #name #ty_generics #where_clause {
            fn from_packet(packet: &crate::ssdp::packet::Packet) -> Result<Self, crate::Error> {
                #(#variants)*;

                Err(crate::Error::UnknownPacket)
            }
        }
    };
    Ok(tokens)
}
