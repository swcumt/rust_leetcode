pub mod subject1;
pub mod subject2;

use crate::subject1::solution::Solution;

fn main() {
    println!("Hello, world!");
    let nums = vec![2, 3, 4, 1, 9];
    let result = Solution::two_sum(nums, 5);
    println!("result is {:?}", result)
}
