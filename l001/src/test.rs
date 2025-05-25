use std::vec;

pub struct InputOutput {
    pub nums: Vec<i32>,
    pub target: i32,
    pub output: Vec<i32>,
}

pub fn get_test_arrs() -> Vec<InputOutput> {
    let test_cases = vec![
        // Trường hợp 1: Trường hợp cơ bản với giải pháp duy nhất
        InputOutput {
            nums: vec![2, 7, 11, 15],
            target: 9,
            output: vec![0, 1], // nums[0] + nums[1] = 2 + 7 = 9
        },
        // Trường hợp 2: Các số âm
        InputOutput {
            nums: vec![-3, 4, 3, 90],
            target: 0,
            output: vec![0, 2], // nums[0] + nums[2] = -3 + 3 = 0
        },
        // Trường hợp 3: Mảng dài hơn với nhiều cặp tiềm năng, nhưng chỉ cần cặp đầu tiên
        InputOutput {
            nums: vec![3, 2, 4, 3],
            target: 6,
            output: vec![1, 2], // nums[1] + nums[2] = 2 + 4 = 6
        },
        // Trường hợp 4: Chỉ có hai phần tử
        InputOutput {
            nums: vec![1, 2],
            target: 3,
            output: vec![0, 1], // nums[0] + nums[1] = 1 + 2 = 3
        },
        // Trường hợp 5: Các số lớn
        InputOutput {
            nums: vec![1000000, 2000000, 3000000],
            target: 5000000,
            output: vec![1, 2], // nums[1] + nums[2] = 2000000 + 3000000 = 5000000
        },
        // Trường hợp 6: Cặp số trùng nhau
        InputOutput {
            nums: vec![3, 3],
            target: 6,
            output: vec![0, 1], // nums[0] + nums[1] = 3 + 3 = 6
        },
    ];
    test_arrs
}
