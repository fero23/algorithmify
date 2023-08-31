use algorithmify::Interpreter;
use algorithmify_macros::define_function_builder;

#[define_function_builder]
fn insertion_sort_pre_condition(i: usize) -> bool {
    i == 1
}

#[define_function_builder]
fn insertion_sort_maintenance_condition<T: PartialOrd>(i: usize, vector: Vec<T>) -> bool {
    let mut valid = true;

    for j in 1..i {
        if vector[j - 1] > vector[j] {
            valid = false;
        }
    }

    valid
}

#[define_function_builder]
fn insertion_sort_post_condition<T>(i: usize, vector: Vec<T>) -> bool {
    i == vector.len()
}

#[define_function_builder {
    main: {
        pre_condition: insertion_sort_pre_condition,
        post_condition: insertion_sort_post_condition,
        maintenance_condition: insertion_sort_maintenance_condition
    }
}]
fn insertion_sort<T>(mut vector: Vec<T>) -> Vec<T>
where
    T: PartialEq + PartialOrd + Copy,
{
    // start on the second element of the vector
    'main: for i in 1..vector.len() {
        let key = vector[i];
        let mut j = i - 1; // start with the element before the selected key

        while j > 0 && vector[j] > key {
            vector[j + 1] = vector[j]; // move larger element to the right one by one
            j = j - 1;
        }

        // insert the key into the space left after moving the larger
        // elements to the right
        vector[j + 1] = key;
    }

    vector
}

#[test]
pub fn test_insertion_sort() {
    let expression = Interpreter::execute_function_with_args(
        insertion_sort__function_builder(),
        vec![vec![3usize, 12, 5, 6].into()],
    )
    .unwrap();

    assert_eq!(insertion_sort(vec![3, 12, 5, 6]), vec![3, 5, 6, 12]);
    assert_eq!(expression, vec![3usize, 5, 6, 12].into());
}
