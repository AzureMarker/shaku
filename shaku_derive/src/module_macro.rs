use crate::debug::get_debug_level;
use crate::error::Error;
use crate::structures::module::ModuleData;
use proc_macro2::TokenStream;

pub fn expand_module_macro(module: ModuleData) -> Result<TokenStream, Error> {
    let debug_level = get_debug_level();
    if debug_level > 1 {
        println!("Module data parsed from input: {:#?}", module);
    }

    // TODO
    let output = quote! {};

    if debug_level > 0 {
        println!("{}", output);
    }

    Ok(output)
}
