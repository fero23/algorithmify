use proc_macro::TokenStream;

mod function_mapper;

#[proc_macro_attribute]
pub fn define_function_builder(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    function_mapper::define_function_builder(stream)
}
