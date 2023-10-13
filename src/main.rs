pub mod subject1;

use subject1::two_sum::Solution;

fn main() {
    println!("Hello, world!");
    let nums = vec![2, 3, 4, 1, 9];
    let result = Solution::two_sum(nums, 5);
    println!("result is {:?}", result)
}
