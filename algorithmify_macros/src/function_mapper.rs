use proc_macro::{Delimiter, TokenStream, TokenTree};

use crate::expression_mapper::map_statements;

#[derive(Default, Debug)]
struct FunctionParams {
    function_name: Option<String>,
    function_statements: Option<String>,
}

pub(crate) fn define_function_builder(stream: TokenStream) -> TokenStream {
    let trees = stream.clone().into_iter().collect::<Vec<_>>();
    let mut params = FunctionParams::default();

    for (index, tree) in trees.iter().enumerate() {
        match tree {
            TokenTree::Ident(identifier) if identifier.to_string() == "fn" => {
                params.function_name = Some(trees[index + 1].to_string());
            }
            TokenTree::Group(body) if body.delimiter() == Delimiter::Brace => {
                map_function_body(&mut params, body);
            }
            _ => {}
        }
    }

    let builder_stream = format!(
        r###"
        fn {}__function_builder() -> algorithmify::Function {{
            algorithmify::Function::new(
                vec![
                    {}
                ]
            )
        }}
    "###,
        params.function_name.unwrap(),
        params.function_statements.unwrap_or("".to_string())
    )
    .parse()
    .unwrap();

    [builder_stream, stream]
        .into_iter()
        .flat_map(|s| s)
        .collect()
}

fn map_function_body(params: &mut FunctionParams, body: &proc_macro::Group) {
    let body = map_statements(body);
    params.function_statements = Some(body);
}
