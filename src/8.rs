struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut start = false;
        let mut chrs = s.chars().fuse();
        let mut neg = false;
        while let Some(c) = chrs.next() {
            //未开始且遇到
            if !start {
                if c == ' ' {
                    continue;
                } else if c == '+' {
                    start = true;
                    continue;
                } else if c == '-' {
                    neg = true;
                    start = true;
                    continue;
                } else if c <= '9' && c >= '0' {
                    start = true;
                } else {
                    return 0;
                }
            }

            //遇到非数字直接返回
            if start {
                if c <= '9' && c >= '0' {
                    let nv = result;
                    let mut no = c.to_digit(10).unwrap() as i32;
                    if neg {
                        no = 0 - no;
                    }
                    if nv > i32::MAX / 10 || (nv == i32::MAX / 10 && no > i32::MAX % 10) {
                        println!("{:?},{:?}", nv, no);
                        return i32::MAX;
                    }
                    if nv < i32::MIN / 10 || (nv == i32::MIN / 10 && no < i32::MIN % 10) {
                        return i32::MIN;
                    }
                    result = nv * 10 + no;
                    //result = result * 10 + c.to_digit(10).unwrap() as i32;
                } else {
                    break;
                }
            }
        }
        //return if neg { -1 * result } else { result };
        return result;
    }
}

fn main() {
    println!("{:?}", i32::MAX);
    println!("{:?}", i32::MIN);
    println!("{:?}", Solution::my_atoi("  -42 abcd".to_string()));
    println!("{:?}", Solution::my_atoi("42".to_string()));
    println!("{:?}", Solution::my_atoi("+-42".to_string()));
    println!("{:?}", Solution::my_atoi("-+42".to_string()));
    println!("{:?}", Solution::my_atoi("2147483646".to_string()));
}
