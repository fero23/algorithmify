use proc_macro::{Delimiter, TokenStream, TokenTree};

#[derive(Default, Debug)]
struct FunctionParams {
    function_name: Option<String>,
    function_statements: Option<String>,
    function_return_expression: Option<String>,
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

    if params.function_name.is_none() || params.function_return_expression.is_none() {
        panic!("Unrecognized function {:?}", trees)
    }

    let builder_stream = format!(
        r###"
        fn {}__function_builder() -> algorithmify::Function {{
            algorithmify::Function::new(
                vec![
                    {}
                ],
                {}
            )
        }}
    "###,
        params.function_name.unwrap(),
        params.function_statements.unwrap_or("".to_string()),
        params.function_return_expression.unwrap()
    )
    .parse()
    .unwrap();

    [builder_stream, stream]
        .into_iter()
        .flat_map(|s| s)
        .collect()
}

fn map_function_body(params: &mut FunctionParams, body: &proc_macro::Group) {
    params.function_statements = Some("".into());
    let body: Vec<TokenTree> = body.stream().into_iter().collect::<Vec<_>>();
    let instructions = body
        .split(|tree| tree.to_string() == ";")
        .collect::<Vec<&[_]>>();

    for instruction in &instructions[..instructions.len() - 1] {
        map_instructions(params, instruction);
    }

    let result = map_expression(instructions[instructions.len() - 1]);
    params.function_return_expression = Some(result);
}

fn map_instructions(params: &mut FunctionParams, instruction: &[TokenTree]) {
    let result = [try_map_assignment]
        .iter()
        .map(|f| f(params, instruction))
        .any(|result| result);

    if !result {
        let reps = instruction
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>();
        panic!("Instruction not recognized: {}", reps.join(" "));
    }
}

fn try_map_assignment(params: &mut FunctionParams, instruction: &[TokenTree]) -> bool {
    if let Some((index, _)) = instruction
        .iter()
        .enumerate()
        .find(|(_, token)| token.to_string() == "=")
    {
        if let TokenTree::Ident(ident) = &instruction[index - 1] {
            let identifier = map_reference(ident);
            let expression = map_expression(&instruction[index + 1..]);

            *params.function_statements.as_mut().unwrap() += &format!(
                "algorithmify::expressions::Statement::Assignment({}, {}),",
                identifier, expression
            );

            return true;
        }
    }

    false
}

fn map_expression(expression: &[TokenTree]) -> String {
    let mut iterator = expression.iter();
    let mut value = map_value(iterator.next().expect("Cannot map empty expression"));

    while let Some(tree) = iterator.next() {
        value = match &tree.to_string()[..] {
            "+" => map_addition(value, map_value(iterator.next().unwrap())),
            "-" => map_substraction(value, map_value(iterator.next().unwrap())),
            "*" => map_multiplication(value, map_value(iterator.next().unwrap())),
            "/" => map_division(value, map_value(iterator.next().unwrap())),
            "&" => map_bitwise_and(value, map_value(iterator.next().unwrap())),
            "|" => map_bitwise_or(value, map_value(iterator.next().unwrap())),
            _ => panic!("Unknown operation '{}'", tree),
        }
    }

    value
}

fn map_addition(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Add({}, {})))", lhs, rhs)
}

fn map_substraction(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Sub({}, {})))", lhs, rhs)
}

fn map_multiplication(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Mul({}, {})))", lhs, rhs)
}

fn map_division(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Div({}, {})))", lhs, rhs)
}

fn map_bitwise_and(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::BitAnd({}, {})))", lhs, rhs)
}

fn map_bitwise_or(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::BitOr({}, {})))", lhs, rhs)
}

fn map_value(tree: &TokenTree) -> String {
    match tree {
        TokenTree::Ident(variable) => map_reference_expression(variable),
        TokenTree::Literal(literal) if literal.to_string().parse::<i32>().is_ok() => {
            map_integer(literal)
        }
        _ => panic!("Unrecognized value '{}'", tree),
    }
}

fn map_integer(literal: &proc_macro::Literal) -> String {
    let number = literal.to_string().parse::<i32>().unwrap();
    format!(
        "algorithmify::expressions::Expression::Integer(algorithmify::expressions::Integer::I32({}))",
        number,
    )
}

fn map_reference_expression(reference: &proc_macro::Ident) -> String {
    format!(
        "algorithmify::expressions::Expression::Reference({})",
        map_reference(reference),
    )
}

fn map_reference(reference: &proc_macro::Ident) -> String {
    format!(
        "algorithmify::expressions::Reference::Variable(\"{}\".to_owned())",
        reference,
    )
}
