//! macros for subscribing to events in [telexide]
//!
//! [telexide]: https://crates.io/crates/telexide

mod structs;
mod utils;

#[allow(unused_extern_crates)]
extern crate proc_macro;

use crate::structs::{BuildableStruct, CommandFunc, ListenerFunc};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use utils::{add_suffix, PunctuatedNamedArgs};

/// A function attribute macro for making event listeners easier.
///
/// This macro transforms an async function into a function returning a pinned
/// box containing a future, which is used internally by telexide to store the
/// function.
#[proc_macro_attribute]
pub fn prepare_listener(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let listener = parse_macro_input!(item as ListenerFunc);
    (quote! {
        #listener
    })
    .into()
}

/// A function attribute macro for making commands.
///
/// This macro will prepare your commands, which can then be added to your
/// framework using the `create_framework!` macro in telexide itself.
///
/// # Options
///
/// To alter how the macro will interpret the command, you can provide options
/// as arguments provided to the macro. ```rust,ignore
/// #[command(description = "the command description")]
/// async fn hello(ctx: Context, message: Message) { ... }
/// ```
/// 
/// | Option      | Usage                            | Description
/// |
/// |-------------|----------------------------------|---------------------------------------------------------------------------------------------|
/// | Description | description = "your description" | The description of the
/// command as to be displayed in telegram, 3-256 characters             |
/// | Name        | name = "the command name"        | The name to be used
/// within telegram, 1-32 characters                                        |
///
/// # Notes
///
/// - The description argument is required, because telegram requires it for a
///   command to be displayed there.
/// - The name argument defaults to the name of the command if not provided
#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let command_fun = parse_macro_input!(item as CommandFunc);
    let args: PunctuatedNamedArgs = parse_macro_input!(attr as PunctuatedNamedArgs);

    let mut telegram_command_name = command_fun.name.to_string();
    let mut description = String::new();

    for arg in args.0 {
        match arg.name.as_str() {
            "name" => telegram_command_name = arg.value.clone(),
            "description" => description = arg.value.clone(),
            _ => (),
        }
    }

    if description.len() < 3 {
        panic!(
            "No description longer than 3 characters has been provided for the {} command, while descriptions are required by telegram",
            telegram_command_name
        )
    }

    let fun_name = command_fun.name.clone();
    let command_name = add_suffix(&fun_name, "COMMAND");
    let options_name = add_suffix(&fun_name, "COMMAND_OPTIONS");

    let command_cooked = command_fun.cooked.clone();
    let options_cooked = command_cooked.clone();

    let command_struct_path = quote!(telexide::framework::types::TelegramCommand);
    let options_struct_path = quote!(telexide::framework::types::CommandOptions);
    let default_command_type_path = quote!(telexide::framework::types::CommandTypes::Default);

    (quote! {
        #(#options_cooked)*
        pub static #options_name: #options_struct_path = #options_struct_path {
            name: #telegram_command_name,
            description: #description,
        };

        #(#command_cooked)*
        pub static #command_name: #command_struct_path = #command_struct_path {
            options: &#options_name,
            command: #default_command_type_path(#fun_name),
        };

        #command_fun
    })
    .into()
}

#[proc_macro_attribute]
pub fn build_struct(_: TokenStream, item: TokenStream) -> TokenStream {
    let build_struct = parse_macro_input!(item as BuildableStruct);

    (quote! {
        #build_struct
    })
    .into()
}
