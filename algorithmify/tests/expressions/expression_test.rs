use algorithmify::{Expression, Interpreter};
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
    assert_eq!(expression, Expression::Integer(6i64.into()));
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
    assert_eq!(expression, Expression::Integer(6.into()));
}
