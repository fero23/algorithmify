use algorithmify::{Expression, Interpreter};
use algorithmify_macros::define_function_builder;

#[test]
pub fn test_for_loop() {
    #[define_function_builder]
    fn for_loop() -> usize {
        let mut acc = 10;

        for i in 1..10 {
            acc = acc + i;
        }

        acc
    }

    let expression = Interpreter::execute_function(for_loop__function_builder()).unwrap();

    assert_eq!(for_loop(), 55);
    assert_eq!(expression, Expression::Integer(55i64.into()));
}

#[test]
pub fn test_for_loop_variable_bounds() {
    #[define_function_builder]
    fn for_loop() -> usize {
        let mut acc = 10;
        let start = 1;
        let end = 10;

        for i in start..end {
            acc = acc + i;
        }

        acc
    }

    let expression = Interpreter::execute_function(for_loop__function_builder()).unwrap();

    assert_eq!(for_loop(), 55);
    assert_eq!(expression, Expression::Integer(55i64.into()));
}

#[test]
pub fn test_for_loop_variable_reassignment() {
    #[allow(unused_assignments)]
    #[define_function_builder]
    fn for_loop() -> usize {
        let i = 1;

        for mut i in 1..10 {
            i = i * 2;
        }

        i
    }

    let expression: Expression =
        Interpreter::execute_function(for_loop__function_builder()).unwrap();

    assert_eq!(for_loop(), 1);
    assert_eq!(expression, Expression::Integer(1.into()));
}

#[test]
pub fn test_while_loop() {
    #[define_function_builder]
    fn while_loop() -> usize {
        let mut acc = 1;

        while acc < 10 {
            acc = acc + 1;
        }

        acc
    }

    let expression = Interpreter::execute_function(while_loop__function_builder()).unwrap();

    assert_eq!(while_loop(), 10);
    assert_eq!(expression, Expression::Integer(10.into()));
}
