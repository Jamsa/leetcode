/*
 * 最长公共前缀
 * https://leetcode-cn.com/problems/longest-common-prefix/
 */


struct Solution {}

impl Solution {

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_str = strs.iter().min_by(|x,y|{
            let xc = x.chars().count();
            let yc = y.chars().count();
            return xc.cmp(&yc);
        });
        let min_len = min_str.unwrap().chars().count();

        let start:usize = 0;
        let mut last_str:&str = "";
        for i in 1..=min_len {
            let idx = i as usize;
            let first: &str = &strs[0][start..idx];
            //println!("x:{:?},{:?}",&strs[0][start..idx],first);
            let all_eq = strs.iter().all(|x| {
                //println!("b:{:?}:{:?}:{:?}",i, x[start..idx], first[start..idx]);
                //println!("b:{:?}",&x[start..idx]);
                &x[start..idx] == first
            });
            //println!("{:?}",all_eq);
            if !all_eq{
                return String::from(last_str);
            }
            last_str = first;
        }
        return String::from(last_str);
    }


}

fn main() {
    println!(
        "{:?}",
        Solution::longest_common_prefix(vec![String::from("flower"),String::from("flow"),String::from("flight")])
    );

    println!(
        "{:?}",
        Solution::longest_common_prefix(vec![String::from("a")])
    );

}
