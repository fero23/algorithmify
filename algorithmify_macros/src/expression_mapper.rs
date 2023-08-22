use proc_macro::{Delimiter, TokenTree};

use crate::{loop_mapper::map_for_loop, token_iterator::TokenIterator};

pub(crate) struct ExpressionMapping {
    pub(crate) mapping: String,
    pub(crate) needs_semicolon_unless_final: bool,
}

pub(crate) fn alt(
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

pub fn map_statements(body: &proc_macro::Group) -> String {
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

pub(crate) fn map_value(tree: &TokenTree) -> Option<String> {
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

pub(crate) fn map_integer(literal: &proc_macro::Literal) -> String {
    let number = literal.to_string().parse::<i32>().unwrap();
    format!(
        "algorithmify::expressions::Expression::Integer(algorithmify::expressions::Integer::I32({}))",
        number,
    )
}

pub(crate) fn map_reference_expression(reference: &proc_macro::Ident) -> String {
    match &*reference.to_string() {
        "true" => "algorithmify::expressions::Expression::Bool(true)".to_owned(),
        "false" => "algorithmify::expressions::Expression::Bool(false)".to_owned(),
        _ => format!(
            "algorithmify::expressions::Expression::Reference({})",
            map_reference(&reference),
        ),
    }
}

pub(crate) fn map_reference(reference: &proc_macro::Ident) -> String {
    format!(
        "algorithmify::expressions::Reference::Variable(\"{}\".to_owned())",
        reference,
    )
}
