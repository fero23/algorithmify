use proc_macro::TokenTree;

use crate::{
    expression_mapper::{map_expression, map_reference, try_get_indexed_access},
    token_iterator::TokenIterator,
};

#[derive(Debug)]
pub(crate) struct StatementMapping {
    pub(crate) mapping: String,
}

pub(crate) fn map_statement(buffer: &mut String, iterator: &mut TokenIterator) {
    let result = [
        try_map_assignment,
        try_map_indexed_assignment,
        try_map_expression,
    ]
    .iter()
    .map(|f| execute_assignment_mapping(iterator, f))
    .find(|result| result.is_some())
    .and_then(|result| result);

    if let Some(result) = result {
        *buffer += result.mapping.as_str();
        *buffer += ",";
    } else {
        if let Some(tokens) = iterator.next_nth(5) {
            let reps = tokens.iter().map(|i| i.to_string()).collect::<Vec<_>>();
            panic!("statement not recognized: {}", reps.join(" "));
        } else {
            panic!("end of statement reached unexpectedly");
        }
    }
}

fn execute_assignment_mapping(
    iterator: &mut TokenIterator,
    mapping: &fn(&mut TokenIterator) -> Option<StatementMapping>,
) -> Option<StatementMapping> {
    let index = iterator.index;
    let result = mapping(iterator);
    if result.is_none() {
        iterator.rewind_to(index);
    }
    return result;
}

fn try_map_assignment(iterator: &mut TokenIterator) -> Option<StatementMapping> {
    iterator.try_get_next_token("let");
    iterator.try_get_next_token("mut");

    let identifier = if let TokenTree::Ident(ident) = iterator.next()? {
        map_reference(ident)
    } else {
        return None;
    };

    iterator.try_get_next_token("=")?;
    let expression = map_expression(iterator)?;

    iterator.try_get_next_token(";")?;

    let mapping = format!(
        "algorithmify::expressions::Statement::Assignment({}, {})",
        identifier, expression.mapping
    );

    Some(StatementMapping { mapping })
}

fn try_map_indexed_assignment(iterator: &mut TokenIterator) -> Option<StatementMapping> {
    iterator.try_get_next_token("let");
    iterator.try_get_next_token("mut");

    let identifier = try_get_indexed_access(iterator)?;

    iterator.try_get_next_token("=")?;
    let expression = map_expression(iterator)?;

    iterator.try_get_next_token(";")?;

    let mapping = format!(
        "algorithmify::expressions::Statement::IndexedAssigment({}, {})",
        identifier, expression.mapping
    );

    Some(StatementMapping { mapping })
}

pub(crate) fn try_map_expression(iterator: &mut TokenIterator) -> Option<StatementMapping> {
    if let Some(expression) = map_expression(iterator) {
        if let (Some(_), Some(_), _) | (None, None, _) | (None, Some(_), false) = (
            iterator.try_get_next_token(";"),
            iterator.peek(),
            expression.needs_semicolon_unless_final,
        ) {
            let mapping = format!(
                "algorithmify::expressions::Statement::Expression({})",
                expression.mapping
            );
            return Some(StatementMapping { mapping });
        }
    }

    None
}
