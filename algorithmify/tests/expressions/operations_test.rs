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

    let context = Interpreter::execute_function(addition__function_builder()).unwrap();

    assert_eq!(addition(), 3);
    assert_eq!(context, Expression::Integer(3.into()));
}

#[test]
fn substraction_test() {
    #[define_function_builder]
    fn substraction() -> i32 {
        let mut a = 6;
        a = a - 2;
        a
    }

    let context = Interpreter::execute_function(substraction__function_builder()).unwrap();

    assert_eq!(substraction(), 4);
    assert_eq!(context, Expression::Integer(4.into()));
}

#[test]
fn multiplication_test() {
    #[define_function_builder]
    fn multiplication() -> i32 {
        let mut a = 3;
        a = a * 2;
        a
    }

    let context = Interpreter::execute_function(multiplication__function_builder()).unwrap();

    assert_eq!(multiplication(), 6);
    assert_eq!(context, Expression::Integer(6.into()));
}

#[test]
fn division_test() {
    #[define_function_builder]
    fn division() -> i32 {
        let mut a = 6;
        a = a / 2;
        a
    }

    let context = Interpreter::execute_function(division__function_builder()).unwrap();

    assert_eq!(division(), 3);
    assert_eq!(context, Expression::Integer(3.into()));
}

#[test]
fn parametherized_expression_test() {
    #[define_function_builder]
    fn parametherized_expression() -> i32 {
        let mut a = 6;
        a = 3 + ((a / 2) * 2) + 3;
        a
    }

    let context =
        Interpreter::execute_function(parametherized_expression__function_builder()).unwrap();

    assert_eq!(parametherized_expression(), 12);
    assert_eq!(context, Expression::Integer(12.into()));
}
