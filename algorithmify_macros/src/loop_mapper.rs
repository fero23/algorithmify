use proc_macro::TokenTree;

use crate::{
    expression_mapper::{
        map_expression, map_integer, map_reference, map_reference_expression, map_statements,
        ExpressionMapping,
    },
    token_iterator::TokenIterator,
};

pub(crate) fn map_for_loop(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
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
        "algorithmify::expressions::Expression::Loop(Box::new(algorithmify::expressions::loops::Loop::RangedFor({}))),",
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

pub(crate) fn map_while_loop(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    iterator.try_get_next_token("while")?;

    let condition = map_expression(iterator)?.mapping;

    let statements = if let TokenTree::Group(group) = iterator.next()? {
        map_statements(group)
    } else {
        return None;
    };

    let while_loop = format!(
        "algorithmify::expressions::loops::WhileLoop {{
            statements: vec![{}],
            condition: {},
        }}",
        statements, condition
    );

    let mapping=  format!(
        "algorithmify::expressions::Expression::Loop(Box::new(algorithmify::expressions::loops::Loop::While({}))),",
        while_loop
    );

    Some(ExpressionMapping {
        mapping,
        needs_semicolon_unless_final: false,
    })
}
