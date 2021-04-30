struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut result = 0;
        let mut v = x;
        while v != 0 {
            let tmp = v % 10;
            if result > i32::MAX / 10
                || (result == i32::MAX / 10 && tmp > i32::MAX % 10)
                || result < i32::MIN / 10
                || (result == i32::MIN / 10 && tmp < i32::MIN % 10)
            {
                return 0;
            }
            result = result * 10 + tmp;
            v /= 10;
        }
        return result;
    }
}

fn main() {
    println!("{:?}", Solution::reverse(123));
}
