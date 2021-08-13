
use std::convert::TryInto;
use std::str;

struct Solution{}

impl Solution {
    pub fn char_index(c:u8, bytes: &[u8])->Option<usize>{
        for(i, &item) in bytes.iter().enumerate(){
            if item == c {
                return Some(i);
            }
        }
        return None;
    }
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = &s.as_bytes();
        //最大长度
        let max_len = bytes.len();
        //最大计数
        let mut max_count = 0;
        //符合条件的字符串的开始位置和结束位置
        let mut begin = 0;
        let mut end = 0;
        //检查起始位置
        let mut pos = 0;
        for(i, &item) in bytes.iter().enumerate(){
            let idx = Self::char_index(item, &bytes[pos..i]);
            match idx {
                Some(index) =>{
                    pos = pos + index +1;
                },
                _ => {
                    //count = count + 1;
                }
            }
            let count = i - pos + 1;
            if count > max_count{
                max_count = count;
                begin = pos;
                end = i+1;
                if end > max_len{
                    end = max_len;
                }
            }
            //println!("from {:?} to {:?} is current substr, length is {:?}, content is {:?},{:?}",
            //         pos,i,count,str::from_utf8(&bytes[pos..i]),idx);
        }

        println!("from {:?} to {:?} is max length substr, length is {:?}, content is {:?}",
                 begin,end,max_count,str::from_utf8(&bytes[begin..end]));

        return max_count.try_into().unwrap();
    }
}

fn main() {
    println!("{:?}",Solution::length_of_longest_substring(String::from("abcbdeghijklmopqrstut")));
    println!("{:?}",Solution::length_of_longest_substring(String::from("abcabcbb")));
    println!("{:?}",Solution::length_of_longest_substring(String::from("bbbbb")));
    println!("{:?}",Solution::length_of_longest_substring(String::from("")));
    println!("{:?}",Solution::length_of_longest_substring(String::from("1")));
}
