mod test;

use std::collections::HashMap;

fn main() {
    let test_arrs = test::get_test_arrs();
    for test_arr in test_arrs {
        println!("{:?}", Solution::two_sum(test_arr.input, test_arr.output));
    }
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = map.get(&complement) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}
