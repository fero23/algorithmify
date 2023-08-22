use std::ops::{Index, Range, RangeFrom, RangeInclusive};

use proc_macro::{Delimiter, TokenStream, TokenTree};

#[derive(Default, Debug)]
struct FunctionParams {
    function_name: Option<String>,
    function_statements: Option<String>,
}

#[derive(Debug)]
enum TokenContainer<'a> {
    Slice(&'a [TokenTree]),
    Vec(Vec<TokenTree>),
}

impl<'a> TokenContainer<'a> {
    fn len(&self) -> usize {
        match self {
            Self::Slice(slice) => slice.len(),
            Self::Vec(vec) => vec.len(),
        }
    }
}

impl<'a> Index<usize> for TokenContainer<'a> {
    type Output = TokenTree;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Slice(slice) => &slice[index],
            Self::Vec(vec) => &vec[index],
        }
    }
}

impl<'a> Index<Range<usize>> for TokenContainer<'a> {
    type Output = [TokenTree];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        match self {
            Self::Slice(slice) => &slice[index],
            Self::Vec(vec) => &vec[index],
        }
    }
}

impl<'a> Index<RangeFrom<usize>> for TokenContainer<'a> {
    type Output = [TokenTree];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        match self {
            Self::Slice(slice) => &slice[index],
            Self::Vec(vec) => &vec[index],
        }
    }
}

impl<'a> Index<RangeInclusive<usize>> for TokenContainer<'a> {
    type Output = [TokenTree];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        match self {
            Self::Slice(slice) => &slice[index],
            Self::Vec(vec) => &vec[index],
        }
    }
}

#[derive(Debug)]
struct TokenIterator<'a> {
    tokens: TokenContainer<'a>,
    index: usize,
}

impl From<Vec<TokenTree>> for TokenIterator<'static> {
    fn from(value: Vec<TokenTree>) -> Self {
        TokenIterator {
            tokens: TokenContainer::Vec(value),
            index: 0,
        }
    }
}

impl<'a> From<&'a [TokenTree]> for TokenIterator<'a> {
    fn from(value: &'a [TokenTree]) -> Self {
        TokenIterator {
            tokens: TokenContainer::Slice(value),
            index: 0,
        }
    }
}

impl<'a> TokenIterator<'a> {
    fn rewind_to(&mut self, index: usize) {
        self.index = index;
    }

    fn peek(&self) -> Option<&TokenTree> {
        if self.index >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.index])
        }
    }

    fn next(&mut self) -> Option<&TokenTree> {
        if self.index >= self.tokens.len() {
            None
        } else {
            let next = Some(&self.tokens[self.index]);
            self.index += 1;
            next
        }
    }

    fn try_get_next_token(&mut self, token: &str) -> Option<()> {
        if self.index < self.tokens.len() && self.tokens[self.index].to_string() == token {
            self.index += 1;
            Some(())
        } else {
            None
        }
    }

    fn next_nth(&mut self, count: usize) -> Option<&[TokenTree]> {
        if self.index < self.tokens.len() {
            let end_index = if self.index + count < self.tokens.len() {
                self.index + count
            } else {
                self.tokens.len()
            };
            let slice = &self.tokens[self.index..end_index];
            self.index = end_index;
            Some(slice)
        } else {
            None
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

    fn get_until_delimiter(&mut self, token: &str) -> Option<&[TokenTree]> {
        if let Some((index, _)) = self.tokens[self.index..]
            .iter()
            .enumerate()
            .find(|(_, t)| t.to_string() == token)
        {
            let slice = &self.tokens[self.index..self.index + index];
            self.index += slice.len() + 1;
            Some(slice)
        } else {
            None
        }
    }
}

struct ExpressionMapping {
    mapping: String,
    needs_semicolon_unless_final: bool,
}

fn alt(
    iterator: &mut TokenIterator,
    functions: &[fn(&mut TokenIterator) -> Option<ExpressionMapping>],
) -> Option<ExpressionMapping> {
    let start_index = iterator.index;

    for function in functions {
        let result = function(iterator);
        if result.is_some() {
            return result;
        } else {
            iterator.index = start_index;
        }
    }
    None
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

fn map_statements(body: &proc_macro::Group) -> String {
    let body: Vec<TokenTree> = body.stream().into_iter().collect::<Vec<_>>();
    let mut iterator: TokenIterator = body.into();

    let mut body = String::new();
    while iterator.peek().is_some() {
        map_statement(&mut body, &mut iterator);
    }
    body
}

fn map_statement(buffer: &mut String, iterator: &mut TokenIterator) {
    let result: bool = [try_map_assignment, try_map_expression]
        .iter()
        .map(|f| f(buffer, iterator))
        .any(|result| result);

    if !result {
        if let Some(tokens) = iterator.next_nth(5) {
            let reps = tokens.iter().map(|i| i.to_string()).collect::<Vec<_>>();
            panic!("statement not recognized: {}", reps.join(" "));
        } else {
            panic!("end of statement reached unexpectedly");
        }
    }
}

fn map_for_loop(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    iterator.try_get_next_token("for")?;

    iterator.try_get_next_token("mut");

    let variable = if let Some(TokenTree::Ident(variable)) = iterator.next() {
        map_reference(variable)
    } else {
        return None;
    };

    iterator.try_get_next_token("in")?;

    let start = map_for_loop_boundary_expression(iterator.next().cloned()?)?;

    iterator.try_get_next_token(".")?;
    iterator.try_get_next_token(".")?;

    let end = map_for_loop_boundary_expression(iterator.next().cloned()?)?;

    let statements = if let TokenTree::Group(group) = iterator.next()? {
        map_statements(group)
    } else {
        return None;
    };

    let for_loop = format!(
        "algorithmify::expressions::loops::RangedForLoop {{
        statements: vec![{}],
        variable: {},
        start: {},
        end: {}
    }}",
        statements, variable, start, end
    );

    let mapping=  format!(
        "algorithmify::expressions::Expression::Loop(Box::new(algorithmify::expressions::loops::Loop::RangedForLoop({}))),",
        for_loop
    );

    Some(ExpressionMapping {
        mapping,
        needs_semicolon_unless_final: false,
    })
}

fn map_for_loop_boundary_expression(expression: TokenTree) -> Option<String> {
    match expression {
        TokenTree::Ident(reference) => Some(map_reference_expression(&reference)),
        TokenTree::Literal(literal) if literal.to_string().parse::<i32>().is_ok() => {
            Some(map_integer(&literal))
        }
        _ => None,
    }
}

fn try_map_assignment(buffer: &mut String, iterator: &mut TokenIterator) -> bool {
    let start_index = iterator.index;

    let statement = iterator.get_until_delimiter(";");
    let find_result = statement.and_then(|statement| {
        statement
            .iter()
            .enumerate()
            .find(|(_, token)| token.to_string() == "=")
            .map(|(index, _)| (statement, index))
    });

    if let Some((statement, index)) = find_result {
        if let TokenTree::Ident(ident) = &statement[index - 1] {
            let identifier = map_reference(ident);
            let mut expression_iterator: TokenIterator = statement[index + 1..].into();

            if let Some(expression) = map_expression(&mut expression_iterator) {
                *buffer += &format!(
                    "algorithmify::expressions::Statement::Assignment({}, {}),",
                    identifier, expression.mapping
                );

                return true;
            }
        }
    }

    iterator.rewind_to(start_index);
    false
}

fn try_map_expression(buffer: &mut String, iterator: &mut TokenIterator) -> bool {
    let start_index = iterator.index;

    if let Some(expression) = map_expression(iterator) {
        if let (Some(_), Some(_), _) | (None, None, _) | (None, Some(_), false) = (
            iterator.try_get_next_token(";"),
            iterator.peek(),
            expression.needs_semicolon_unless_final,
        ) {
            *buffer += &format!(
                "algorithmify::expressions::Statement::Expression({}),",
                expression.mapping
            );
            return true;
        }
    }

    iterator.rewind_to(start_index);
    false
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

fn map_simple_expression(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    if let Some(mapping) = map_fourth_tier_precedence_expression(iterator) {
        Some(ExpressionMapping {
            mapping,
            needs_semicolon_unless_final: true,
        })
    } else {
        None
    }
}

fn map_expression(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    alt(iterator, &[map_for_loop, map_simple_expression])
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
            let mut iterator: TokenIterator = children.into();
            match (map_expression(&mut iterator), iterator.next()) {
                (Some(result), None) => Some(result.mapping),
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
