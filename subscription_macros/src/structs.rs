use proc_macro2::TokenStream as TokenStream2;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Ident, Stmt, Visibility, ReturnType, braced, FnArg, Token, Block};
use quote::{quote, ToTokens};
use super::utils::ParenthesisedItems;

#[derive(Debug)]
pub struct ListenerFunc {
    /// `#[...]`-style attributes.
    pub attributes: Vec<Attribute>,
    /// Populated by `#[cfg(...)]` type attributes.
    pub cooked: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Ident,
    pub args: Vec<FnArg>,
    pub body: Vec<Stmt>,
}

impl Parse for ListenerFunc {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;

        let (cooked, attributes): (Vec<_>, Vec<_>) =
            attributes.into_iter().partition(|a| a.path.is_ident("cfg"));

        let visibility = input.parse::<Visibility>()?;

        input.parse::<Token![async]>()?;
        input.parse::<Token![fn]>()?;
        let name = input.parse()?;

        let ParenthesisedItems(args) = input.parse::<ParenthesisedItems<FnArg>>()?;

        match input.parse::<ReturnType>()? {
            ReturnType::Type(_, _) => {
                return Err(input
                    .error("expected a default return value"))
            },
            ReturnType::Default => ()
        };

        let body_content;
        braced!(body_content in input);
        let body: Vec<Stmt> = body_content.call(Block::parse_within)?;

        let args = args.into_iter().collect::<Vec<FnArg>>();

        Ok(Self {
            attributes,
            cooked,
            visibility,
            name,
            args,
            body,
        })
    }
}

impl ToTokens for ListenerFunc {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Self {
            attributes: _,
            cooked,
            visibility,
            name,
            args,
            body,
        } = self;

        stream.extend(quote! {
            #(#cooked)*
            #visibility fn #name (#(#args),*) -> ::std::pin::Pin<::std::boxed::Box<(dyn ::std::future::Future<Output = ()> + ::std::marker::Send )>> {
                ::std::boxed::Box::pin(async move {
                    #(#body)*
            })
            }
        });
    }
}

#[derive(Debug)]
pub struct CommandFunc {
    /// `#[...]`-style attributes.
    pub attributes: Vec<Attribute>,
    /// Populated by `#[cfg(...)]` type attributes.
    pub cooked: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Ident,
    pub args: Vec<FnArg>,
    pub body: Vec<Stmt>,
}

impl Parse for CommandFunc {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;

        let (cooked, attributes): (Vec<_>, Vec<_>) =
            attributes.into_iter().partition(|a| a.path.is_ident("cfg"));

        let visibility = input.parse::<Visibility>()?;

        input.parse::<Token![async]>()?;
        input.parse::<Token![fn]>()?;
        let name = input.parse()?;

        let ParenthesisedItems(args) = input.parse::<ParenthesisedItems<FnArg>>()?;

        match input.parse::<ReturnType>()? {
            ReturnType::Type(_, _) => {
                return Err(input
                    .error("expected a default return value"))
            },
            ReturnType::Default => ()
        };

        let body_content;
        braced!(body_content in input);
        let body: Vec<Stmt> = body_content.call(Block::parse_within)?;

        let args = args.into_iter().collect::<Vec<FnArg>>();

        Ok(Self {
            attributes,
            cooked,
            visibility,
            name,
            args,
            body,
        })
    }
}

impl ToTokens for CommandFunc {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Self {
            attributes: _,
            cooked,
            visibility,
            name,
            args,
            body,
        } = self;

        stream.extend(quote! {
            #(#cooked)*
            #visibility fn #name (#(#args),*) -> ::std::pin::Pin<::std::boxed::Box<(dyn ::std::future::Future<Output = ()> + ::std::marker::Send )>> {
                ::std::boxed::Box::pin(async move {
                    #(#body)*
            })
            }
        });
    }
}
