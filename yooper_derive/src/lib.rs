extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Lit, MetaNameValue, Result, Token};

// #[devire(ToMessage)]
// enum Packet {
//   #[message(reqline = Notify, sts = "ssdp:alive")]
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
    sts: Option<Lit>,
}

impl Parse for VariantCondition {
    fn parse(input: ParseStream) -> Result<Self> {
        let attr_args: Punctuated<MetaNameValue, Token![,]> =
            Punctuated::parse_separated_nonempty(input)?;

        let mut reqline = None;
        let mut sts = None;

        let span = attr_args.span();

        for arg in attr_args {
            if arg.path.is_ident("reqline") {
                reqline = Some(arg.lit);
            } else if arg.path.is_ident("sts") {
                sts = Some(arg.lit);
            }
        }

        Ok(Self {
            reqline: reqline
                .ok_or_else(|| Error::new(span, "Missing required attribute arg reqline"))?,
            sts,
        })
    }
}

impl ToTokens for VariantCondition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let reqline = &self.reqline;
        tokens.extend(quote! { msg.reqline == #reqline});
        if let Some(sts) = &self.sts {
            tokens.extend(quote! { msg.headers.get("sts").ok_or("") == #sts })
        }
    }
}

fn derive_message_impl(input: DeriveInput) -> Result<TokenStream> {
    let name = input.ident;

    let enums = match input.data {
        Data::Enum(e) => e,
        _ => unimplemented!(),
    };

    let mut stream = TokenStream::new();

    for variant in enums.variants {
        let vname = variant.ident;
        let attr = variant
            .attrs
            .into_iter()
            .find(|v| v.path.is_ident("message"));
        if let Some(attr) = attr {
            let cond: VariantCondition = attr.parse_args()?;

            stream.extend(quote! {
                if #cond {
                    return Ok(
                        #name::#vname(
                        ..Default::defaults()
                        )
                    )
                }
            });
        }
    }

    Ok(stream)
}
