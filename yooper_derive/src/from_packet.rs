use crate::ast::{parse_variants, MessageVariant, VariantMember};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, Result};

// #[derive(ToMessage)]
// enum Packet {
//   #[message(reqline = Notify, nts = "ssdp:alive")]
//   Alive {
//     # #[header("ssdpuuid.upnp.org")
//     uuid: String
//   }
//  }
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

        tokens.extend(quote! { packet.typ == crate::PacketType::#reqline});
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
            fields,
            ..
        } = &self.0;
        let fields = fields.iter().map(VariantMember::from_message); // TODO

        let cond = self.as_from_condition();
        tokens.extend(quote! {
            if #cond {
                return Ok(
                    #parent::#name {
                        #(#fields),*
                    }
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
                #ident: packet.headers.get(#header).map_or(Ok(None), |v| v.parse().map(Some))?
            }
        } else {
            quote! {
                #ident: packet.headers.get(#header).ok_or_else(|| crate::Error::MissingHeader(#header))?.parse()?
            }
        };

        tokens.extend(q);
    }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let variants = parse_variants(input.clone())?; // TODO(EKF)
    let variants: Vec<FromPacket> = variants.iter().map(MessageVariant::from_message).collect();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = input.ident;

    let tokens = quote! {
        #[automatically_derived]
        impl #impl_generics crate::FromPacket for #name #ty_generics #where_clause {
            fn from_packet(packet: &crate::Packet) -> Result<Self, crate::Error> {
                #(#variants)*;

                Err(crate::Error::UnknownPacket)
            }
        }
    };
    Ok(tokens)
}
