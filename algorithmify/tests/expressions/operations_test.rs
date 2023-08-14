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
