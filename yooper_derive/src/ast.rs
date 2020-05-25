use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, Data, DeriveInput, Error, Field, Fields,
    Ident, Lit, LitStr, MetaNameValue, Path, Result, Token, Type, Variant,
};

pub struct MessageVariant {
    pub parent: Ident,
    pub name: Ident,
    pub reqline: Ident,
    pub nts: Option<Lit>,
    pub struct_name: Path,
}

fn parse_annotation(attr: Attribute) -> Result<(Ident, Option<Lit>)> {
    let attr_args: Punctuated<MetaNameValue, Token![,]> =
        attr.parse_args_with(Punctuated::parse_separated_nonempty)?;

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

    let reqline =
        reqline.ok_or_else(|| Error::new(span, "Missing required attribute arg reqline"))?;
    Ok((reqline_to_ident(reqline)?, nts))
}

fn reqline_to_ident(lit: Lit) -> Result<Ident> {
    let span = lit.span();
    let litstr = match lit {
        Lit::Str(v) => Ok(v),
        _ => Err(Error::new(span, "reqline should be a PacketType")),
    }?;

    Ok(Ident::new(&litstr.value(), Span::call_site()))
}

impl MessageVariant {
    fn from_variant(parent: &Ident, variant: Variant) -> Result<Option<Self>> {
        let span = variant.span();
        let name = variant.ident;
        let parent = parent.clone(); // TODO: EKF

        let attr = variant
            .attrs
            .into_iter()
            .find(|v| v.path.is_ident("message"));

        let attr = match attr {
            Some(v) => v,
            None => return Ok(None),
        };

        if variant.fields.len() != 1 {
            return Err(Error::new(span, "Only singleton variants supported"));
        }

        let field = variant.fields.iter().next().unwrap();
        let struct_name = match &field.ty {
            Type::Path(p) => p.path.clone(),
            _ => return Err(Error::new(span, "Expected path member for enum variant")),
        };

        let (reqline, nts) = parse_annotation(attr)?;

        Ok(Some(Self {
            parent,
            name,
            reqline,
            nts,
            struct_name,
        }))
    }
}

pub fn parse_variants(input: DeriveInput) -> Result<Vec<MessageVariant>> {
    let enums = match input.data {
        Data::Enum(e) => e,
        _ => return Err(Error::new(input.span(), "Only Enums make sense here!")),
    };

    let name = input.ident;
    let variants: Result<Vec<Option<MessageVariant>>> = enums
        .variants
        .into_iter()
        .map(|v| MessageVariant::from_variant(&name, v))
        .collect();
    Ok(variants?.into_iter().filter_map(|v| v).collect())
}

fn path_is_option(path: &Path) -> bool {
    path.segments.len() == 1 && path.segments.iter().next().unwrap().ident == "Option"
}

pub struct MessageStruct {
    pub name: Ident,
    pub fields: Vec<VariantMember>,
}

pub fn parse_header_struct(input: DeriveInput) -> Result<MessageStruct> {
    let span = input.span();
    let name = input.ident;
    let strct = match input.data {
        Data::Struct(e) => e,
        _ => {
            return Err(Error::new(
                span,
                "Only Struct derives have been implemented!",
            ))
        }
    };

    let fields = match strct.fields {
        Fields::Named(f) => Ok(f),
        _ => Err(Error::new(
            strct.struct_token.span,
            "only named structs supported",
        )),
    }?
    .named
    .into_iter()
    .map(VariantMember::from_field)
    .collect::<Result<Vec<_>>>()?;

    Ok(MessageStruct { name, fields })
}

pub struct VariantMember {
    pub optional: bool,
    pub header: String,
    pub ident: Ident,
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
