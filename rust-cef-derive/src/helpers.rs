use crate::proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::convert::From;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DeriveInput, Error as SynError, Expr, ExprLit, Lit, LitStr, Meta,
    MetaNameValue, Token,
};

pub const CEF_ATTRIBUTE_APPLICATION: &str = "This attribute only applies to Structs or Enums.";

pub type ParseAttrResult<T> = Result<T, TokenStream2>;

pub fn is_valid_item_type(item: &DeriveInput) -> Option<TokenStream> {
    // Only applies to structs and enums
    match item.data {
        Data::Struct(_) | Data::Enum(_) => {}
        _ => {
            return Some(TokenStream::from(
                SynError::new(Span::call_site(), CEF_ATTRIBUTE_APPLICATION).to_compile_error(),
            ))
        }
    }

    None
}

// Helps cut through a lot of parse tree and doesn't confuse reading-context
pub fn parse_attrs_to_name_value(
    attr: &Attribute,
    message: &str,
) -> ParseAttrResult<Vec<MetaNameValue>> {
    let mut mnvs: Vec<MetaNameValue> = vec![];

    match &attr.meta {
        Meta::List(_) => {
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .map_err(|e| e.to_compile_error())?;
            for nestedmeta in nested {
                match nestedmeta {
                    Meta::NameValue(mnv) => {
                        mnvs.push(mnv);
                    }
                    _ => {
                        return Err(
                            SynError::new(attr.span(), message.to_owned()).to_compile_error()
                        )
                    }
                }
            }
        }
        _ => return Err(SynError::new(attr.span(), message.to_owned()).to_compile_error()),
    }

    Ok(mnvs)
}

/// Returns the string literal if the expression is one (e.g. the `"value"` in `Name = "value"`).
pub fn expr_to_lit_str(expr: &Expr) -> Option<&LitStr> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(strval),
            ..
        }) => Some(strval),
        _ => None,
    }
}
