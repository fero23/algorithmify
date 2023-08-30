use proc_macro::{Delimiter, TokenStream, TokenTree};

use crate::{
    expression_mapper::{map_statements, try_get_identifier},
    token_iterator::TokenIterator,
};

#[derive(Default, Debug)]
struct FunctionParams {
    function_name: Option<String>,
    function_args: Option<String>,
    function_statements: Option<String>,
}

pub(crate) fn define_function_builder(stream: TokenStream, attrs: TokenStream) -> TokenStream {
    let trees = stream.clone().into_iter().collect::<Vec<_>>();
    let mut params = FunctionParams::default();

    for (index, tree) in trees.iter().enumerate() {
        match tree {
            TokenTree::Ident(identifier) if identifier.to_string() == "fn" => {
                params.function_name = Some(trees[index + 1].to_string());
            }
            TokenTree::Group(body) if body.delimiter() == Delimiter::Parenthesis => {
                params.function_args = map_args(body);
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
                ],
                vec![
                    {}
                ],
                std::collections::HashMap::from([{}])
            )
        }}

        #[allow(unused_labels)]
        #[allow(dead_code)]
    "###,
        params.function_name.unwrap(),
        params.function_args.unwrap_or("".to_string()),
        params.function_statements.unwrap_or("".to_string()),
        build_contracts(attrs)
    )
    .parse()
    .unwrap();

    [builder_stream, stream]
        .into_iter()
        .flat_map(|s| s)
        .collect()
}

fn map_args(body: &proc_macro::Group) -> Option<String> {
    let mut iterator: TokenIterator = body.stream().into_iter().collect::<Vec<_>>().into();

    let mut args = Vec::new();
    while let Some(_) = iterator.peek() {
        let arg = try_get_identifier(&mut iterator)?;
        iterator.try_get_next_token(":")?;
        try_get_identifier(&mut iterator)?;
        args.push(format!("\"{}\".to_owned()", arg));
        iterator.try_get_next_token(",");
    }

    Some(args.join(", "))
}

fn map_function_body(params: &mut FunctionParams, body: &proc_macro::Group) {
    let body = map_statements(body);
    params.function_statements = Some(body);
}

fn build_contracts(attrs: TokenStream) -> String {
    let mut iterator: TokenIterator = attrs.into_iter().collect::<Vec<_>>().into();
    let mut contracts = String::new();
    while let Some(tag) = iterator.next().map(|t| t.to_string()) {
        iterator
            .try_get_next_token(":")
            .expect(&format!("expected a :, got {:?}", iterator.peek()));

        match iterator.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                let inner_iterator = group.stream().into_iter().collect::<Vec<_>>().into();
                contracts += &build_contract(tag, inner_iterator);
                contracts += ",";
            }
            other => panic!("expected a braced group, got '{:?}'", other),
        }

        iterator.try_get_next_token(",");
    }

    contracts
}

fn build_contract(tag: String, mut iterator: TokenIterator) -> String {
    let mut conditions = Vec::new();

    while let Some(condition) = iterator.next().map(|t| t.to_string()) {
        iterator
            .try_get_next_token(":")
            .expect(&format!("expected a :, got {:?}", iterator.peek()));

        match iterator.next() {
            Some(TokenTree::Ident(function)) => {
                conditions.push(format!(
                    "{}: Some((\"{}\".to_owned(), {}__function_builder))",
                    condition, function, function
                ));
            }
            other => panic!("expected a braced group, got '{:?}'", other),
        }

        iterator.try_get_next_token(",");
    }

    conditions.push("..Default::default()".to_owned());

    format!(
        "(\"{}\".to_owned(), algorithmify::expressions::loops::Contract {{{}}})",
        tag,
        conditions.join(",")
    )
}
