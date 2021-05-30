/*
 * 电话号码的字母组合
 * https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/
 */
struct Solution {}

//可换成map
const BASES : &'static [&'static str] = &["","","abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];

impl Solution {
    pub fn combination(lrow: &Vec<String>,ritem: &str) -> Vec<String>{
        let mut result:Vec<String> = vec![];

        for item in lrow.iter(){
            for rc in ritem.chars(){
                let mut row = item.clone();
                row.push(rc);
                result.push(row);
            }
        }

        return result;
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result:Vec<String> = vec![];
        let mut keys:Vec<&str> = vec![];
        digits.chars().for_each(|x|{
            let idx = x.to_string().parse::<i32>().unwrap();
            let idx = idx as usize;
            let key_str: &str = BASES[idx];
            keys.push(key_str);
        });

        for item in keys.iter(){
            if result.is_empty(){
                result.push(String::from(""));
            }
            result = Self::combination(&result, item);
        }

        return result;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::letter_combinations(String::from("23"))
    );
    println!(
        "{:?}",
        Solution::letter_combinations(String::from(""))
    );
    println!(
        "{:?}",
        Solution::letter_combinations(String::from("2"))
    );
}

