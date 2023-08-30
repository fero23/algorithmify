use algorithmify::Interpreter;
use algorithmify_macros::define_function_builder;

#[test]
pub fn test_basic_if_condition() {
    #[define_function_builder]
    fn if_condition() -> usize {
        let mut result = 1;

        if result < 10 {
            result = result + 1;
        }

        result
    }

    let expression = Interpreter::execute_function(if_condition__function_builder()).unwrap();

    assert_eq!(if_condition(), 2);
    assert_eq!(expression, 2.into());
}

#[test]
pub fn test_if_else_condition() {
    #[define_function_builder]
    fn if_else_condition() -> usize {
        let mut result = 1;

        if result == 2 {
            result = result + 1;
        } else {
            result = result + 2;
        }

        result
    }

    let expression = Interpreter::execute_function(if_else_condition__function_builder()).unwrap();

    assert_eq!(if_else_condition(), 3);
    assert_eq!(expression, 3.into());
}

#[test]
pub fn test_else_if_condition() {
    #[define_function_builder]
    fn else_if_condition() -> usize {
        let mut result = 1;

        if result == 2 {
            result = result + 1;
        } else if result == 1 {
            result = result + 2;
        } else {
            result = 10;
        }

        result
    }

    let expression = Interpreter::execute_function(else_if_condition__function_builder()).unwrap();

    assert_eq!(else_if_condition(), 3);
    assert_eq!(expression, 3.into());
}
