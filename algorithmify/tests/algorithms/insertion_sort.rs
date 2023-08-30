use algorithmify::Interpreter;
use algorithmify_macros::define_function_builder;

#[test]
pub fn test_insertion_sort() {
    #[define_function_builder]
    fn pre_condition() -> bool {
        true
    }

    #[define_function_builder]
    fn maintenance_condition() -> bool {
        true
    }

    #[define_function_builder]
    fn post_condition() -> bool {
        true
    }

    #[define_function_builder {
        main: {
            pre_condition: pre_condition,
            post_condition: post_condition,
            maintenance_condition: maintenance_condition
        }
    }]
    fn insertion_sort<T: PartialEq + PartialOrd>(vector: Vec<T>) -> Vec<T> {
        vector
    }

    let expression = Interpreter::execute_function_with_args(
        insertion_sort__function_builder(),
        vec![vec![3usize, 12, 5, 6].into()],
    )
    .unwrap();

    assert_eq!(insertion_sort(vec![3, 12, 5, 6]), vec![3, 5, 6, 12]);
    assert_eq!(expression, 3.into());
}
