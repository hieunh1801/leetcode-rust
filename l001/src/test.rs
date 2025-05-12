use std::vec;

pub struct InputOutput {
    pub input: Vec<i32>,
    pub output: i32,
}

pub fn get_test_arrs() -> Vec<InputOutput> {
    let test_arrs = vec![
        InputOutput {
            input: vec![2, 7, 11, 15],
            output: 9,
        },
        InputOutput {
            input: vec![3, 2, 4],
            output: 6,
        },
        InputOutput {
            input: vec![3, 3],
            output: 6,
        },
    ];
    test_arrs
}
