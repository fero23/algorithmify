use algorithmify::Interpreter;
use algorithmify_macros::define_function_builder;

#[test]
pub fn test_vector() {
    #[define_function_builder]
    fn vector() -> usize {
        let mut vector = vec![1, 2, 3];
        for i in 0..3 {
            vector[i] = (i + 1) * 2;
        }
        vector[2]
    }

    let expression = Interpreter::execute_function(vector__function_builder()).unwrap();

    assert_eq!(vector(), 6);
    assert_eq!(expression, 6i64.into());
}

#[test]
pub fn test_vector_copy() {
    #[define_function_builder]
    fn vector_copy() -> usize {
        let vector = vec![1, 2, 3];
        let mut other_vector = vec![0; 3];
        for i in 0..3 {
            other_vector[i] = vector[i] * 2;
        }
        other_vector[2]
    }

    let expression = Interpreter::execute_function(vector_copy__function_builder()).unwrap();

    assert_eq!(vector_copy(), 6);
    assert_eq!(expression, 6.into());
}

#[test]
pub fn test_function_call() {
    #[define_function_builder]
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    #[define_function_builder]
    fn function_call() -> i32 {
        let a = sum(1, 2);
        sum(a, 2)
    }

    let expression = Interpreter::execute_function(function_call__function_builder()).unwrap();

    assert_eq!(function_call(), 5);
    assert_eq!(expression, 5.into());
}
