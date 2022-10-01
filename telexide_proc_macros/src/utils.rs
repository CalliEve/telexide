use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    Field, GenericArgument, Path, PathArguments, PathSegment, Token, Type, TypePath,
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

        Ok(Self { name, value })
    }
}

pub fn add_suffix(ident: &Ident, suffix: &str) -> Ident {
    format_ident!("{}_{}", ident.to_string(), suffix)
}

#[derive(Debug)]
pub struct BuildImplBlock {
    mandatory_fields: Vec<(Ident, Type)>,
    settable_fields: Vec<(Ident, Type)>,
    struct_name: Ident,
}

impl BuildImplBlock {
    pub fn new(fields: Vec<Field>, struct_name: Ident) -> Result<Self> {
        let fields: Vec<(Ident, Type, BuildFieldType)> = fields
            .into_iter()
            .map(|field| {
                let ident = field.ident.ok_or_else(|| {
                    syn::Error::new(Span::call_site(), "expected a struct with named fields")
                })?;

                let (ty, field_type) = if let Type::Path(TypePath {
                    path: Path { segments, .. },
                    ..
                }) = &field.ty
                {
                    if let Some(PathSegment {
                        ident,
                        arguments: PathArguments::AngleBracketed(args),
                    }) = segments.first()
                    {
                        if ident.to_string() == "Option" {
                            if let Some(GenericArgument::Type(ty)) = args.args.first().clone() {
                                (ty.clone(), BuildFieldType::Settable)
                            } else {
                                (field.ty, BuildFieldType::Mandatory)
                            }
                        } else {
                            (field.ty, BuildFieldType::Mandatory)
                        }
                    } else {
                        (field.ty, BuildFieldType::Mandatory)
                    }
                } else {
                    (field.ty, BuildFieldType::Mandatory)
                };

                Ok((ident, ty, field_type))
            })
            .collect::<Result<_>>()?;

        let (mandatory_fields, settable_fields) = fields.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut mandatory, mut settable), (i, ty, field_type)| {
                if field_type == BuildFieldType::Mandatory {
                    mandatory.push((i, ty))
                } else {
                    settable.push((i, ty))
                }
                (mandatory, settable)
            },
        );

        Ok(Self {
            mandatory_fields,
            settable_fields,
            struct_name,
        })
    }
}

impl ToTokens for BuildImplBlock {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let Self {
            mandatory_fields,
            settable_fields,
            struct_name,
        } = self;

        let new_fields = fields_to_tokenstreams(mandatory_fields, |(ident, ty)| {
            if ty.to_token_stream().to_string() == "String" {
                quote! {#ident:#ident.to_string()}
            } else {
                quote! {#ident:#ident}
            }
        });

        let mandatory_fields = fields_to_tokenstreams(mandatory_fields, |(ident, ty)| {
            if ty.to_token_stream().to_string() == "String" {
                quote! {#ident: impl ToString}
            } else {
                quote! {#ident:#ty}
            }
        });

        let set_function_names = fields_to_tokenstreams(settable_fields, |(ident, _)| {
            let ident = format_ident!("set_{}", ident);
            quote! {#ident}
        });

        let settable_names = fields_to_tokenstreams(settable_fields, |(ident, _)| quote! {#ident});

        let field_setting = fields_to_tokenstreams(settable_fields, |(ident, ty)| {
            if ty.to_token_stream().to_string() == "String" {
                quote! {#ident.to_string()}
            } else {
                quote! {#ident}
            }
        });

        let settable_fields = fields_to_tokenstreams(settable_fields, |(ident, ty)| {
            if ty.to_token_stream().to_string() == "String" {
                quote! {#ident: impl ToString}
            } else {
                quote! {#ident:#ty}
            }
        });

        stream.extend(quote! {
            impl #struct_name {
                pub fn new(#(#mandatory_fields),*) -> Self {
                    Self {
                        #(#new_fields,)*
                        #(#settable_names: std::default::Default::default()),*
                    }
                }

                #(pub fn #set_function_names(&mut self, #settable_fields) -> &mut Self {
                    self.#settable_names = Some(#field_setting);
                    self
                })*
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BuildFieldType {
    Mandatory,
    Settable,
}

fn fields_to_tokenstreams<F>(fields: &Vec<(Ident, Type)>, func: F) -> Vec<TokenStream>
where
    F: FnMut(&(Ident, Type)) -> TokenStream,
{
    fields.iter().map(func).collect()
}
