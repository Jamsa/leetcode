/*
 * 罗马数字转整数
 * https://leetcode-cn.com/problems/roman-to-integer/
 */


struct Solution {}

//可换成map
const RBASES : &'static [&'static str] = &["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"];
const BASES : &'static [ i32] = &[1000,900,500,400,100,90,50,40,10,9,5,4,1];

impl Solution {

    pub fn seg_value(s: &str)->Option<i32>{
        let bases = BASES;
        let rbases = RBASES;

        let pos = rbases.iter().position(|x| *x == s);
        if pos.is_some(){
            let index = pos.unwrap();
            let base: i32 = bases[index];
            Some(base)
        }else{
            None
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut chars = s.chars();
        let mut result: i32 = 0;
        let mut c1:Option<char>=None;
        let mut c2:Option<char>=None;
        loop{
            if c1==None{
                c1 = chars.next();
            }
            if c2==None{
                c2 = chars.next();
            }

            if c1.is_none() {
                break;
            }
            let seg1 = c1.unwrap().to_string();
            // 两个字符都有值
            if c2.is_some(){
                let seg2 = c2.unwrap().to_string();
                let mut roman: String = seg1.clone();
                roman.push_str(&seg2);

                let value = Self::seg_value(&roman);
                if value.is_some(){
                    // 能找到两个字符对应的数字
                    result += value.unwrap();
                    c1 = None;
                    c2 = None;
                }else{
                    // 找不到两字符，则只找第一个字符
                    let value = Self::seg_value(&seg1);
                    if value.is_some(){
                        result += value.unwrap();
                        c1 = c2;
                        c2 = None;
                    }
                }
            }else{
                // 最后一个字符
                let value = Self::seg_value(&seg1);
                if value.is_some(){
                    result += value.unwrap();
                    c1 = None;
                }
            }
        }

        result
    }

}

fn main() {
    println!(
        "{:?}",
        Solution::roman_to_int("XXII".to_owned())
    );
    println!(
        "{:?}",
        Solution::roman_to_int("MCMXCIV".to_owned())
    );
    println!(
        "{:?}",
        Solution::roman_to_int("CDI".to_owned())
    );
    println!(
        "{:?}",
        Solution::roman_to_int("IV".to_owned())
    );
}
