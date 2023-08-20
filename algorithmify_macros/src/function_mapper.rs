use proc_macro::{Delimiter, TokenStream, TokenTree};

#[derive(Default, Debug)]
struct FunctionParams {
    function_name: Option<String>,
    function_statements: Option<String>,
}

#[derive(Debug)]
struct TokenIterator {
    tokens: Vec<TokenTree>,
    index: usize,
}

impl<T> From<T> for TokenIterator
where
    T: Iterator<Item = TokenTree>,
{
    fn from(value: T) -> Self {
        TokenIterator {
            tokens: value.collect(),
            index: 0,
        }
    }
}

impl TokenIterator {
    fn rewind_to(&mut self, index: usize) {
        self.index = index;
    }

    fn next(&mut self) -> Option<&TokenTree> {
        if self.index == self.tokens.len() {
            None
        } else {
            let next = Some(&self.tokens[self.index]);
            self.index += 1;
            next
        }
    }

    fn next_nth_string(&mut self, count: usize) -> Option<String> {
        if self.index + count >= self.tokens.len() {
            None
        } else {
            let result = (self.index..self.index + count).fold(String::new(), |acc, index| {
                acc + &self.tokens[index].to_string()
            });
            self.index += count;
            Some(result)
        }
    }
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
    params.function_statements = Some("".into());
    let body: Vec<TokenTree> = body.stream().into_iter().collect::<Vec<_>>();
    let statements = body
        .split(|tree| tree.to_string() == ";")
        .collect::<Vec<&[_]>>();

    for statement in &statements {
        map_statements(params, statement);
    }
}

fn map_statements(params: &mut FunctionParams, statement: &[TokenTree]) {
    let result: bool = [try_map_assignment, try_map_expression]
        .iter()
        .map(|f| f(params, statement))
        .any(|result| result);

    if !result {
        let reps = statement.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        panic!("statement not recognized: {}", reps.join(" "));
    }
}

fn try_map_assignment(params: &mut FunctionParams, statement: &[TokenTree]) -> bool {
    if let Some((index, _)) = statement
        .iter()
        .enumerate()
        .find(|(_, token)| token.to_string() == "=")
    {
        if let TokenTree::Ident(ident) = &statement[index - 1] {
            let identifier = map_reference(ident);
            let mut iterator: TokenIterator = statement[index + 1..].iter().cloned().into();

            if let Some(expression) = map_expression(&mut iterator) {
                *params.function_statements.as_mut().unwrap() += &format!(
                    "algorithmify::expressions::Statement::Assignment({}, {}),",
                    identifier, expression
                );

                return true;
            }
        }
    }

    false
}

fn try_map_expression(params: &mut FunctionParams, statement: &[TokenTree]) -> bool {
    let mut iterator: TokenIterator = statement.iter().cloned().into();

    match (map_expression(&mut iterator), iterator.next()) {
        (Some(expression), None) => {
            *params.function_statements.as_mut().unwrap() += &format!(
                "algorithmify::expressions::Statement::Expression({}),",
                expression
            );
            true
        }
        _ => false,
    }
}

fn map_first_tier_precedence_expression(iterator: &mut TokenIterator) -> Option<String> {
    let mut index = iterator.index;
    let token = iterator.next()?;

    if let Some(mut lhs) = map_value(token) {
        index = iterator.index;

        while let Some(operator) = iterator.next().map(|t| t.to_string()) {
            let token = iterator.next()?;
            let rhs = map_value(token);

            if let Some(rhs) = rhs {
                lhs = match &*operator {
                    "*" => map_multiplication(lhs, rhs),
                    "/" => map_division(lhs, rhs),
                    "&" => map_bitwise_and(lhs, rhs),
                    "|" => map_bitwise_or(lhs, rhs),
                    _ => break,
                };

                index = iterator.index;
            } else {
                break;
            }
        }

        iterator.rewind_to(index);
        return Some(lhs);
    }

    iterator.rewind_to(index);
    None
}

fn map_second_tier_precedence_expression(iterator: &mut TokenIterator) -> Option<String> {
    let mut index = iterator.index;

    if let Some(mut lhs) = map_first_tier_precedence_expression(iterator) {
        index = iterator.index;

        while let Some(operator) = iterator.next().map(|t| t.to_string()) {
            let rhs = map_first_tier_precedence_expression(iterator);
            if let Some(rhs) = rhs {
                lhs = match &*operator {
                    "+" => map_addition(lhs, rhs),
                    "-" => map_substraction(lhs, rhs),
                    _ => break,
                };

                index = iterator.index;
            } else {
                break;
            }
        }

        iterator.rewind_to(index);
        return Some(lhs);
    }

    iterator.rewind_to(index);
    None
}

fn map_third_tier_precedence_expression(iterator: &mut TokenIterator) -> Option<String> {
    let mut index = iterator.index;

    if let Some(mut lhs) = map_second_tier_precedence_expression(iterator) {
        index = iterator.index;

        while let Some(operator) = iterator.next_nth_string(2) {
            let rhs = map_second_tier_precedence_expression(iterator);
            if let Some(rhs) = rhs {
                lhs = match &*operator {
                    "==" => map_eq(lhs, rhs),
                    "!=" => map_ne(lhs, rhs),
                    ">=" => map_gte(lhs, rhs),
                    "<=" => map_lte(lhs, rhs),
                    _ => break,
                };

                index = iterator.index;
            } else {
                break;
            }
        }

        iterator.rewind_to(index);

        while let Some(operator) = iterator.next().map(|t| t.to_string()) {
            let rhs = map_third_tier_precedence_expression(iterator);
            if let Some(rhs) = rhs {
                lhs = match &*operator {
                    ">" => map_gt(lhs, rhs),
                    "<" => map_lt(lhs, rhs),
                    _ => break,
                };

                index = iterator.index;
            } else {
                break;
            }
        }

        iterator.rewind_to(index);
        return Some(lhs);
    }

    iterator.rewind_to(index);
    None
}

fn map_fourth_tier_precedence_expression(iterator: &mut TokenIterator) -> Option<String> {
    let mut index = iterator.index;

    if let Some(mut lhs) = map_third_tier_precedence_expression(iterator) {
        index = iterator.index;

        while let Some(operator) = iterator.next_nth_string(2) {
            let rhs = map_third_tier_precedence_expression(iterator);
            if let Some(rhs) = rhs {
                lhs = match &*operator {
                    "&&" => map_logical_and(lhs, rhs),
                    "||" => map_logical_or(lhs, rhs),
                    _ => break,
                };

                index = iterator.index;
            }
        }

        iterator.rewind_to(index);
        return Some(lhs);
    }

    iterator.rewind_to(index);
    None
}

fn map_expression(iterator: &mut TokenIterator) -> Option<String> {
    map_fourth_tier_precedence_expression(iterator)
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

fn map_logical_and(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::And({}, {})))", lhs, rhs)
}

fn map_logical_or(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Or({}, {})))", lhs, rhs)
}

fn map_eq(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Eq({}, {})))", lhs, rhs)
}

fn map_ne(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Ne({}, {})))", lhs, rhs)
}

fn map_lt(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Lt({}, {})))", lhs, rhs)
}

fn map_lte(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Lte({}, {})))", lhs, rhs)
}

fn map_gt(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Gt({}, {})))", lhs, rhs)
}

fn map_gte(lhs: String, rhs: String) -> String {
    format!("algorithmify::expressions::Expression::Operation(Box::new(algorithmify::expressions::Operation::Gte({}, {})))", lhs, rhs)
}

fn map_value(tree: &TokenTree) -> Option<String> {
    match tree {
        TokenTree::Ident(variable) => Some(map_reference_expression(variable)),
        TokenTree::Literal(literal) if literal.to_string().parse::<i32>().is_ok() => {
            Some(map_integer(literal))
        }
        TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
            let children = group.stream().into_iter().collect::<Vec<_>>();
            let mut iterator: TokenIterator = children.into_iter().into();
            match (map_expression(&mut iterator), iterator.next()) {
                (Some(result), None) => Some(result),
                _ => None,
            }
        }
        _ => None,
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
    match &*reference.to_string() {
        "true" => "algorithmify::expressions::Expression::Bool(true)".to_owned(),
        "false" => "algorithmify::expressions::Expression::Bool(false)".to_owned(),
        _ => format!(
            "algorithmify::expressions::Expression::Reference({})",
            map_reference(&reference),
        ),
    }
}

fn map_reference(reference: &proc_macro::Ident) -> String {
    format!(
        "algorithmify::expressions::Reference::Variable(\"{}\".to_owned())",
        reference,
    )
}
