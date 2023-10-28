pub struct Solution;

/*
Given a string s, find the length of the longest 
substring
 without repeating characters.

 

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.
Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.
Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
*/

impl Solution {

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0;
        let mut current_length = 0;
        let mut index = 0;
        let s_slice: Vec<char> = s.chars().collect();
        let mut store = std::collections::HashMap::new();
        while index < s_slice.len() {
            if let Some(&i) = store.get(&s_slice[index]) {
                current_length = 0;
                store.clear();
                index = i + 1;
                continue;
            } else {
                store.insert(&s_slice[index], index);
                current_length += 1;
            }
            index += 1;
            if current_length > max_length {
                max_length = current_length;
            }
        }
        max_length
    }
}

#[cfg(test)]
mod test {
    use super::Solution;


    #[test]
    fn test_ex1() {
        let len = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(len, 3);
    }

    #[test]
    fn test_ex2() {
        let len = Solution::length_of_longest_substring("bbbbb".to_string());
        assert_eq!(len, 1);
    }

    #[test]
    fn test_ex3() {
        let len = Solution::length_of_longest_substring("pwwkew".to_string());
        assert_eq!(3, len)
    }
}
