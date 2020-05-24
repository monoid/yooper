extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Data, DeriveInput, Error, Field, Fields, Ident, Lit, LitStr, MetaNameValue, Path, Result,
    Token, Type,
};

// #[devire(ToMessage)]
// enum Packet {
//   #[message(reqline = Notify, nts = "ssdp:alive")]
//   Alive {
//     # #[header("ssdpuuid.upnp.org")
//     uuid: String
//   }
//  }
// }

#[proc_macro_derive(FromPacket, attributes(header, message))]
pub fn derive_from_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let e = parse_macro_input!(input as DeriveInput);
    derive_message_impl(e)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

struct VariantCondition {
    reqline: Lit,
    nts: Option<Lit>,
}

impl Parse for VariantCondition {
    fn parse(input: ParseStream) -> Result<Self> {
        let attr_args: Punctuated<MetaNameValue, Token![,]> =
            Punctuated::parse_separated_nonempty(input)?;

        let mut reqline = None;
        let mut nts = None;

        let span = attr_args.span();

        for arg in attr_args {
            if arg.path.is_ident("reqline") {
                reqline = Some(arg.lit);
            } else if arg.path.is_ident("nts") {
                nts = Some(arg.lit);
            }
        }

        Ok(Self {
            reqline: reqline
                .ok_or_else(|| Error::new(span, "Missing required attribute arg reqline"))?,
            nts,
        })
    }
}

impl ToTokens for VariantCondition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.reqline.span();
        let reqline = match &self.reqline {
            Lit::Str(v) => v,
            _ => {
                tokens
                    .extend(Error::new(span, "reqline should be a PacketType").to_compile_error());
                return;
            }
        };
        let reqline_ident = Ident::new(&reqline.value(), Span::call_site());
        tokens.extend(quote! { packet.typ == crate::PacketType::#reqline_ident});
        if let Some(nts) = &self.nts {
            tokens.extend(quote! {
                && packet.headers.get("nts").map_or(false, |h| h == #nts )
            })
        }
    }
}

struct VariantMember {
    optional: bool,
    header: String,
    ident: Ident,
}

impl VariantMember {
    fn from_field(field: Field) -> Result<Self> {
        let span = field.span();
        let ident = field
            .ident
            .ok_or_else(|| Error::new(span, "unnamed fields not supported"))?;
        let attr = field.attrs.iter().find(|a| a.path.is_ident("header"));
        let header = match attr {
            Some(attr) => {
                let lit: LitStr = attr.parse_args()?;
                lit.value()
            }
            None => ident.to_string(),
        };

        let optional = match field.ty {
            Type::Path(t) => path_is_option(&t.path),
            _ => false,
        };

        Ok(Self {
            optional,
            header,
            ident,
        })
    }
}

impl ToTokens for VariantMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VariantMember { header, ident, .. } = &self;

        let q = if self.optional {
            quote! {
                #ident: packet.headers.get(#header).map_or(Ok(None), |v| v.parse().map(Some))?
            }
        } else {
            quote! {
                #ident: packet.headers.get(#header).ok_or_else(|| crate::Error::MissingHeader(#header))?.parse()?
            }
        };
        tokens.extend(q)
    }
}

fn path_is_option(path: &Path) -> bool {
    path.segments.len() == 1 && path.segments.iter().next().unwrap().ident == "Option"
}

fn derive_message_impl(input: DeriveInput) -> Result<TokenStream> {
    let name = input.ident;

    let enums = match input.data {
        Data::Enum(e) => e,
        _ => unimplemented!(),
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut stream = TokenStream::new();

    for variant in enums.variants {
        let span = variant.span();
        let vname = variant.ident;
        let attr = variant
            .attrs
            .into_iter()
            .find(|v| v.path.is_ident("message"));
        if let Some(attr) = attr {
            let cond: VariantCondition = attr.parse_args()?;

            let fields = match variant.fields {
                Fields::Named(f) => Ok(f),
                _ => Err(Error::new(span, "only named Enums supported")),
            }?
            .named
            .into_iter()
            .map(VariantMember::from_field)
            .collect::<Result<Vec<_>>>()?;

            stream.extend(quote! {
                if #cond {
                    return Ok(
                        #name::#vname {
                            #(#fields),*
                        }
                    )
                }
            });
        }
    }

    let tokens = quote! {
        #[automatically_derived]
        impl #impl_generics crate::FromPacket for #name #ty_generics #where_clause {
            fn from_packet(packet: &crate::Packet) -> Result<Self, crate::Error> {
                #stream

                Err(crate::Error::UnknownPacket)
            }
        }
    };
    Ok(tokens)
}
