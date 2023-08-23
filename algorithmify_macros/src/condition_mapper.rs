use proc_macro::TokenTree;

use crate::{
    expression_mapper::{alt, map_block, map_expression, map_statements, ExpressionMapping},
    token_iterator::TokenIterator,
};

pub(crate) fn map_if_condition(iterator: &mut TokenIterator) -> Option<ExpressionMapping> {
    iterator.try_get_next_token("if")?;

    let condition = map_expression(iterator)?.mapping;

    let statements = if let TokenTree::Group(group) = iterator.next()? {
        map_statements(group)
    } else {
        return None;
    };

    let else_clause = if iterator.try_get_next_token("else").is_some() {
        let else_expression = alt(iterator, &[map_block, map_if_condition])?;
        format!("Some({})", else_expression.mapping)
    } else {
        "None".to_owned()
    };

    let if_condition = format!(
        "algorithmify::expressions::conditions::If {{
            statements: vec![{}],
            condition: {},
            else_clause: {}
        }}",
        statements, condition, else_clause
    );

    let mapping=  format!(
        "algorithmify::expressions::Expression::Condition(Box::new(algorithmify::expressions::conditions::Condition::If({}))),",
        if_condition
    );

    Some(ExpressionMapping {
        mapping,
        needs_semicolon_unless_final: false,
    })
}
