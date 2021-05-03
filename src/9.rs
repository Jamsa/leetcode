/*
回文数
https://leetcode-cn.com/problems/palindrome-number/
*/

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s = x.to_string();
        let bytes = s.as_bytes();
        let len = bytes.len();
        for i in 0..len / 2 {
            let a = bytes[i];
            let b = bytes[len - i - 1];
            if a != b {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("{:?}", Solution::is_palindrome(121));
    println!("{:?}", Solution::is_palindrome(11));
    println!("{:?}", Solution::is_palindrome(-121));
}
