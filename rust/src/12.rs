/*
 * 整数转罗马数字
 * https://leetcode-cn.com/problems/integer-to-roman/
 */


struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let bases = vec![1000,900,500,400,100,90,50,40,10,9,5,4,1];
        let rbases = vec!["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"];
        let mut result = String::from("");
        let mut n = num;

        for (idx,base) in bases.iter().enumerate(){
            let int_part = n / base;
            n = n % base;
            for _i in 0..int_part {
                result += rbases[idx];
            }
            if num == 0{
                break;
            }
        }
        return result.to_string();
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::int_to_roman(1994)
    );
    println!(
        "{:?}",
        Solution::int_to_roman(11)
    );
    println!(
        "{:?}",
        Solution::int_to_roman(58)
    );
    println!(
        "{:?}",
        Solution::int_to_roman(1994)
    );
}
