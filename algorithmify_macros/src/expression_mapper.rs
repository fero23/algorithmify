use std::str::FromStr;

use proc_macro::{Delimiter, TokenTree};

use crate::{
    condition_mapper::map_if_condition,
    loop_mapper::{map_for_loop, map_while_loop},
    statement_mapper::map_statement,
    token_iterator::TokenIterator,
};

#[derive(Debug)]
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

pub(crate) fn map_statements(body: &proc_macro::Group) -> String {
    let body: Vec<TokenTree> = body.stream().into_iter().collect::<Vec<_>>();
    let mut iterator: TokenIterator = body.into();

    let mut body = String::new();
    while iterator.peek().is_some() {
        map_statement(&mut body, &mut iterator);
    }
    body
}

fn map_first_tier_precedence_expression(iterator: &mut TokenIterator) -> Option<String> {
    let mut index = iterator.index;

    if let Some(mut lhs) = map_value(iterator) {
        index = iterator.index;

        while let Some(operator) = iterator.next().map(|t| t.to_string()) {
            let rhs = map_value(iterator);

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

fn map_vec_sequence(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    iterator.try_get_next_token("vec")?;
    iterator.try_get_next_token("!")?;

    let expressions = match iterator.next()? {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            let mut expressions = String::new();
            let tokens = group.stream().into_iter().collect::<Vec<_>>();
            let mut iterator: TokenIterator = tokens.into();

            while let Some(result) = map_expression(&mut iterator) {
                expressions += result.mapping.as_str();
                expressions += ",";
                iterator.try_get_next_token(",");
            }

            if iterator.next().is_some() {
                return None;
            }

            expressions
        }
        _ => return None,
    };

    let mapping = format!(
        "algorithmify::expressions::Expression::Vector(vec![{}])",
        expressions
    );

    Some(ExpressionMapping {
        mapping,
        needs_semicolon_unless_final: false,
    })
}

fn map_function_call(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    let identifier = try_get_identifier(iterator)?;
    match iterator.next()? {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
            let mut args_iterator: TokenIterator =
                group.stream().into_iter().collect::<Vec<_>>().into();

            let mut expressions = String::new();

            while let Some(expression) = map_expression(&mut args_iterator) {
                expressions += &expression.mapping;
                expressions += ",";

                args_iterator.try_get_next_token(",");
            }

            let mapping = format!(
                "algorithmify::expressions::Expression::FunctionCall(algorithmify::expressions::FunctionCall{{
                    builder: {}__function_builder, 
                    params: vec![{}]
                }})",
                identifier, expressions
            );

            Some(ExpressionMapping {
                mapping,
                needs_semicolon_unless_final: true,
            })
        }
        _ => None,
    }
}

fn map_vec_shorthand(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    iterator.try_get_next_token("vec")?;
    iterator.try_get_next_token("!")?;

    match iterator.next()? {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            let tokens = group.stream().into_iter().collect::<Vec<_>>();
            let mut iterator: TokenIterator = tokens.into();

            let expression = map_expression(&mut iterator)?.mapping;
            iterator.try_get_next_token(";")?;
            let repetitions = try_get::<usize>(&mut iterator)?;

            if iterator.next().is_some() {
                return None;
            }

            let mapping = format!(
                "algorithmify::expressions::Expression::Vector(vec![{};{}])",
                expression, repetitions
            );

            Some(ExpressionMapping {
                mapping,
                needs_semicolon_unless_final: false,
            })
        }
        _ => None,
    }
}

pub(crate) fn map_expression(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    alt(
        iterator,
        &[
            map_function_call,
            map_vec_shorthand,
            map_vec_sequence,
            map_if_condition,
            map_block,
            map_for_loop,
            map_while_loop,
            map_simple_expression,
        ],
    )
}

pub(crate) fn map_block(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    match iterator.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
            let statements = map_statements(group);

            let block = format!(
                "algorithmify::expressions::block::Block {{
                    statements: vec![{}],
                }}",
                statements
            );

            let mapping = format!(
                "algorithmify::expressions::Expression::Block(Box::new({})),",
                block
            );

            Some(ExpressionMapping {
                mapping,
                needs_semicolon_unless_final: false,
            })
        }
        _ => None,
    }
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

pub(crate) fn map_value(iterator: &mut TokenIterator) -> Option<String> {
    let index = iterator.index;

    if let reference @ Some(_) = try_get_indexed_access_expression(iterator) {
        return reference;
    }

    iterator.rewind_to(index);

    match iterator.next()? {
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

pub(crate) fn try_get_indexed_access(iterator: &mut TokenIterator) -> Option<String> {
    if let TokenTree::Ident(ident) = iterator.next().cloned()? {
        if let TokenTree::Group(group) = iterator.next()? {
            if group.delimiter() == Delimiter::Bracket {
                let mut iterator: TokenIterator =
                    group.stream().into_iter().collect::<Vec<_>>().into();
                let index = map_expression(&mut iterator)?;
                if iterator.next().is_none() {
                    return Some(map_indexed_reference(&ident, index.mapping));
                }
            }
        }
    }

    None
}

pub(crate) fn try_get_indexed_access_expression(iterator: &mut TokenIterator) -> Option<String> {
    if let TokenTree::Ident(ident) = iterator.next().cloned()? {
        if let TokenTree::Group(group) = iterator.next()? {
            if group.delimiter() == Delimiter::Bracket {
                let mut iterator: TokenIterator =
                    group.stream().into_iter().collect::<Vec<_>>().into();
                let index = map_expression(&mut iterator)?;
                if iterator.next().is_none() {
                    return Some(map_indexed_reference_expression(&ident, index.mapping));
                }
            }
        }
    }

    None
}

pub(crate) fn try_get<T: FromStr>(iterator: &mut TokenIterator) -> Option<T> {
    match iterator.next()? {
        TokenTree::Literal(literal) => literal.to_string().parse::<T>().ok(),
        _ => None,
    }
}

pub(crate) fn try_get_identifier(iterator: &mut TokenIterator) -> Option<String> {
    match iterator.next()? {
        TokenTree::Ident(identifier) => Some(identifier.to_string()),
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

pub(crate) fn map_indexed_reference_expression(
    reference: &proc_macro::Ident,
    index: String,
) -> String {
    format!(
        "algorithmify::expressions::Expression::IndexedAccessExpression({})",
        map_indexed_reference(&reference, index),
    )
}

pub(crate) fn map_reference(reference: &proc_macro::Ident) -> String {
    format!(
        "algorithmify::expressions::Reference::Variable(\"{}\".to_owned())",
        reference,
    )
}

pub(crate) fn map_indexed_reference(reference: &proc_macro::Ident, index: String) -> String {
    format!(
        "algorithmify::expressions::IndexedAccessExpression{{
            variable: \"{}\".to_owned(), 
            index: Box::new({})
        }}",
        reference, index
    )
}
