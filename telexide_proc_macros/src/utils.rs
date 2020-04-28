use proc_macro2::{Ident, Literal};
use quote::format_ident;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    Token,
};

pub struct ParenthesisedItems<T>(pub Punctuated<T, Comma>);

impl<T: Parse> Parse for ParenthesisedItems<T> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        Ok(Self(content.parse_terminated(T::parse)?))
    }
}

pub struct PunctuatedNamedArgs(pub Punctuated<NamedArgs, Comma>);

impl Parse for PunctuatedNamedArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self(input.parse_terminated(NamedArgs::parse)?))
    }
}

pub struct NamedArgs {
    pub name: String,
    pub value: String,
}

impl Parse for NamedArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse::<Ident>()?.to_string();
        input.parse::<Token![=]>()?;
        let mut value = input.parse::<Literal>()?.to_string();
        value = value.trim_start_matches('\"').to_owned();
        value = value.trim_end_matches('\"').to_owned();

        Ok(Self {
            name,
            value,
        })
    }
}

pub fn add_suffix(ident: &Ident, suffix: &str) -> Ident {
    format_ident!("{}_{}", ident.to_string(), suffix)
}
