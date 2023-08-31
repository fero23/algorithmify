use proc_macro::TokenTree;

use crate::{
    expression_mapper::{map_expression, map_reference, map_statements, ExpressionMapping},
    token_iterator::TokenIterator,
};

pub(crate) fn map_for_loop(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    let tag = try_get_tag(iterator);

    iterator.try_get_next_token("for")?;

    iterator.try_get_next_token("mut");

    let variable = if let Some(TokenTree::Ident(variable)) = iterator.next() {
        map_reference(variable)
    } else {
        return None;
    };

    iterator.try_get_next_token("in")?;

    let start = map_expression(iterator)?.mapping;

    iterator.try_get_next_token(".")?;
    iterator.try_get_next_token(".")?;

    let end = map_expression(iterator)?.mapping;

    let statements = if let TokenTree::Group(group) = iterator.next()? {
        map_statements(group)
    } else {
        return None;
    };

    let for_loop = format!(
        "algorithmify::expressions::loops::RangedForLoop {{
            tag: {},
            statements: vec![{}],
            variable: {},
            start: {},
            end: {}
        }}",
        tag, statements, variable, start, end
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

pub(crate) fn map_while_loop(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    let tag = try_get_tag(iterator);

    iterator.try_get_next_token("while")?;

    let condition = map_expression(iterator)?.mapping;

    let statements = if let TokenTree::Group(group) = iterator.next()? {
        map_statements(group)
    } else {
        return None;
    };

    let while_loop = format!(
        "algorithmify::expressions::loops::WhileLoop {{
            tag: {},
            statements: vec![{}],
            condition: {},
        }}",
        tag, statements, condition
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

pub(crate) fn try_get_tag(iterator: &mut TokenIterator) -> String {
    let index = iterator.index;

    if let (
        Some(TokenTree::Punct(apostrophe)),
        Some(TokenTree::Ident(identifier)),
        Some(TokenTree::Punct(colon)),
    ) = (
        iterator.next().cloned(),
        iterator.next().cloned(),
        iterator.next(),
    ) {
        if apostrophe.as_char() == '\'' && colon.as_char() == ':' {
            return format!("Some(\"{}\".to_owned())", identifier);
        }
    }

    iterator.rewind_to(index);
    "None".to_owned()
}
