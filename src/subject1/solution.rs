use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut store = HashMap::new();
        let mut result = Vec::new(); 
        let default_val = -1;
        for (i, v) in nums.iter().enumerate(){
            let k = target - v;
            let r = store.get(&k).unwrap_or(&default_val);
            if default_val != *r {
                result.push(*r);
                result.push(i as i32);
                return result;
            }
            store.insert(v, i as i32);
        }
        result
    }
}
#[cfg(test)]
mod tests{
    use super::Solution;

    #[test]
    fn test_two_sum_eg1() {
        let r = Solution::two_sum(vec![2, 7, 11, 5], 9);
        assert_eq!(vec![0, 1], r);
    }

    #[test]
    fn test_two_sum_eg2() {
        let r = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(vec![1, 2], r);
    }

    #[test]
    fn test_two_sum_eg3() {
        let r = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(vec![0, 1], r);
    }
}
