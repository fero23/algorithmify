use algorithmify::{Expression, Interpreter};
use algorithmify_macros::define_function_builder;

#[test]
fn addition_test() {
    #[define_function_builder]
    fn addition() -> i32 {
        let mut a = 1;
        a = a + 2;
        a
    }

    let expression = Interpreter::execute_function(addition__function_builder()).unwrap();

    assert_eq!(addition(), 3);
    assert_eq!(expression, 3.into());
}

#[test]
fn substraction_test() {
    #[define_function_builder]
    fn substraction() -> i32 {
        let mut a = 6;
        a = a - 2;
        a
    }

    let expression = Interpreter::execute_function(substraction__function_builder()).unwrap();

    assert_eq!(substraction(), 4);
    assert_eq!(expression, 4.into());
}

#[test]
fn multiplication_test() {
    #[define_function_builder]
    fn multiplication() -> i32 {
        let mut a = 3;
        a = a * 2;
        a
    }

    let expression = Interpreter::execute_function(multiplication__function_builder()).unwrap();

    assert_eq!(multiplication(), 6);
    assert_eq!(expression, 6.into());
}

#[test]
fn division_test() {
    #[define_function_builder]
    fn division() -> i32 {
        let mut a = 6;
        a = a / 2;
        a
    }

    let expression = Interpreter::execute_function(division__function_builder()).unwrap();

    assert_eq!(division(), 3);
    assert_eq!(expression, 3.into());
}

#[test]
fn parametherized_expression_test() {
    #[define_function_builder]
    fn parametherized_expression() -> i32 {
        let mut a = 6;
        a = 3 + ((a / 2) * 2) + 3;
        a
    }

    let expression =
        Interpreter::execute_function(parametherized_expression__function_builder()).unwrap();

    assert_eq!(parametherized_expression(), 12);
    assert_eq!(expression, 12.into());
}

#[test]
fn operator_precedence_test() {
    #[define_function_builder]
    fn operator_precedence() -> i32 {
        let mut a = 6;
        a = 3 + a / 2 * 2 + 3;
        a
    }

    let expression =
        Interpreter::execute_function(operator_precedence__function_builder()).unwrap();

    assert_eq!(operator_precedence(), 12);
    assert_eq!(expression, 12.into());
}

#[test]
fn boolean_logic_test() {
    #[define_function_builder]
    fn boolean_logic_true() -> bool {
        let a = 6;
        let eq = a == 6;
        let ne = a != 5;
        let cmp = 23 > 6 && 11 < 23 && 5 <= 6 && 5 >= 3;

        (eq && ne && cmp) || false
    }

    let expression = Interpreter::execute_function(boolean_logic_true__function_builder()).unwrap();

    assert_eq!(boolean_logic_true(), true);
    assert_eq!(expression, Expression::Bool(true));

    #[define_function_builder]
    fn boolean_logic_false() -> bool {
        let a = 6;
        let eq = a == 6;
        let ne = a != 6;
        let cmp = 23 > 6 && 11 < 23 && 5 <= 6 && 5 >= 3;

        (eq && ne && cmp) || false
    }

    let expression =
        Interpreter::execute_function(boolean_logic_false__function_builder()).unwrap();

    assert_eq!(boolean_logic_false(), false);
    assert_eq!(expression, Expression::Bool(false));
}
