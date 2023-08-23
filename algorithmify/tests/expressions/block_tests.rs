use algorithmify::{Expression, Interpreter};
use algorithmify_macros::define_function_builder;

#[test]
pub fn test_block() {
    #[define_function_builder]
    fn block() -> usize {
        let result = {
            let a = 1;
            a + 2
        };

        result
    }

    let expression = Interpreter::execute_function(block__function_builder()).unwrap();

    assert_eq!(block(), 3);
    assert_eq!(expression, Expression::Integer(3.into()));
}
