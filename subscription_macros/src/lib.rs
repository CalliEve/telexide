mod structs;
mod utils;

use quote::quote;
use proc_macro::TokenStream;
use syn::{
    parse_macro_input
};
use crate::structs::{
    ListenerFunc, CommandFunc
};
use utils::{add_suffix, PunctuatedNamedArgs};

#[proc_macro_attribute]
pub fn prepare_listener(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let listener = parse_macro_input!(item as ListenerFunc);
    (quote!{
        #listener
    }).into()
}

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let command_fun = parse_macro_input!(item as CommandFunc);
    let args: PunctuatedNamedArgs = parse_macro_input!(attr as PunctuatedNamedArgs);

    let mut telegram_command_name = command_fun.name.to_string();
    let mut description = String::new();

    for arg in args.0 {
        match arg.name.as_str() {
            "name" => telegram_command_name =  arg.value.clone(),
            "description" => description = arg.value.clone(),
            _ => ()
        }
    }

    let fun_name = command_fun.name.clone();
    let command_name = add_suffix(&fun_name,"COMMAND");
    let options_name = add_suffix(&fun_name, "COMMAND_OPTIONS");

    let command_cooked = command_fun.cooked.clone();
    let options_cooked = command_cooked.clone();

    let command_struct_path = quote!(telexide::raw_cmd::RawTelegramCommand);
    let options_struct_path = quote!(telexide::framework::types::CommandOptions);
    let default_command_type_path = quote!(telexide::raw_cmd::RawCommandTypes::Default);

    (quote!{
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
    }).into()
}
