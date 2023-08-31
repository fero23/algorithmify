# Algorithmify

This is the main repository for the `algorithmify` experimental library. This library is designed to create specifications for algorithms defined using Rust code.

`algorithmify` is a platform that contains a token parser contained in the `algorithmify_macros` crate. This crate leverages the power of the Rust macro system to define and expose the `define_function_builder` macro that transforms a Rust function into a structural representation (or executable Abstract Syntax Tree) of the function. Such representation can can be executed by the `Interpreter` that lives inside the `algorithmify` crate.

As for right now the `define_function_builder` macro only supports a very basic subset of the Rust syntax. That should be enough to define and test many basic algorithms that can be expressed in `C`-like pseudocode and its equivalent Rust implementation. In the future the plan is to add the calculation of the algorithm complexity and other algorithm stats.

## Function builders

The main entry point for the `algorithmify` interpreter is the `Function` struct. This is a structural representation of a Rust function that has the `define_function_builder` set on it.

The `define_function_builder` declares and defines a function with the same name as the function it's placed on but with the `__function_builder` postfix appeneded to it. Calling this builder functions returns a `Function` instance that can be executed by the interpreter.

The following function definition:

```rust
#[define_function_builder]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

Is expanded to:

```rust
fn sum__function_builder() -> algorithmify::Function {
    algorithmify::Function::new(
        <[_]>::into_vec( // argument list
            #[rustc_box]
            ::alloc::boxed::Box::new(["a".to_owned(), "b".to_owned()]),
        ),
        <[_]>::into_vec( // function body
            #[rustc_box]
            ::alloc::boxed::Box::new([
                algorithmify::expressions::Statement::Expression(
                    algorithmify::expressions::Expression::Operation(
                        Box::new(
                            algorithmify::expressions::Operation::Add(
                                algorithmify::expressions::Expression::Reference(
                                    algorithmify::expressions::Reference::Variable(
                                        "a".to_owned(),
                                    ),
                                ),
                                algorithmify::expressions::Expression::Reference(
                                    algorithmify::expressions::Reference::Variable(
                                        "b".to_owned(),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ]),
        ),
        std::collections::HashMap::from([]), // contract directory
    )
}

#[allow(unused_labels)]
#[allow(dead_code)]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

## Interpreter

As you can see in the previous section, the `define_function_builder` macro defines a builder function that can be
executed like this:

```rust
fn addition_test() {
    #[define_function_builder]
    fn addition() -> i32 {
        let mut a = 1;
        a = a + 2;
        a
    }

    assert_eq!(addition(), 3);

    let expression = Interpreter::execute_function(addition__function_builder()).unwrap();
    assert_eq!(expression, 3.into());
}
```

As you can see above, we execute the native Rust function first, and then we execute the builder function using the in-memory interpreter. The result is the same value of 3 in both cases, because the interpreter executes the equivalent instructions in memory.

## Contracts

This is the main feature of the library. It allows to define a specification of an algorithm using contracts. The contracts are condition sets (pre, post and maintenance) that can be applied to the any of the loops within the algorithm to check its correctness.

The contracts are Rust functions that use the `define_function_builder` macro, and that are included in the contract definition arguments of the `define_function_builder` of the function that the contract is applied to.

The pre-condition is executed before the loop start, the maintenance condition after each loop cycle and post-condition after the loop finishes.

Here is an example of it, checking the correctness of an insertion sort implementation in Rust:

```rust
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
```

The test we have at the end verifies that the end results are the same, but the three `condition` functions we declare as part of the contract are also executed and check the correct state of the algorithm during its execution.
