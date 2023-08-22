use proc_macro::TokenStream;

mod expression_mapper;
mod function_mapper;
mod loop_mapper;
mod token_container;
mod token_iterator;

#[proc_macro_attribute]
pub fn define_function_builder(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    function_mapper::define_function_builder(stream)
}
