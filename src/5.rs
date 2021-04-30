
struct Solution{}

impl Solution {


    // ------ 暴力法 -----

    pub fn is_palindrome2(s: &str) -> bool {
        let bytes = s.as_bytes();
        let len = bytes.len();
        for i in 0 .. len/2{
            let a = bytes[i];
            let b = bytes[len - i - 1];
            if a != b{
                return false;
            }
        }
        true
    }

    pub fn longest_palindrome2(ss: String) -> String {
        let s = &ss;
        let len = s.len();
        let mut result = "";
        let mut max: usize = 0;
        for i in 0 .. len{
            for j in i+1 ..= len{
                let tmp = &s[i..j];
                println!("{:?},{:?},{:?}",tmp,i,j);
                if Solution::is_palindrome2(tmp) && tmp.len()>max{
                    result  = tmp;
                    max = tmp.len();
                }
            }
        }
        String::from(result)
    }

    // ------ 暴力法 -----

    /*pub fn longest_palindrome(s: String) -> String {
        String::from("hello")
    }*/
}

fn main() {
    //println!("{:?}",&"bb"[0..=1]);
    println!("{:?}",Solution::longest_palindrome2(String::from("babad")));
    println!("{:?}",Solution::longest_palindrome2(String::from("bb")));
    println!("{:?}",Solution::longest_palindrome2(String::from("ac")));
}
