/*
 * 括号生成
 * 参考这个链接容易理解 https://leetcode-cn.com/problems/generate-parentheses/solution/sui-ran-bu-shi-zui-xiu-de-dan-zhi-shao-n-0yt3/
 * https://leetcode-cn.com/problems/generate-parentheses/solution/
 */

struct Solution {}

impl Solution {
    pub fn generate_all(current:&mut Vec<char>, pos: usize, result: &mut Vec<String>){
        println!("cap:{:?},pos:{:?},current:{:?}",current.capacity(),pos,current);
        if pos == current.len(){//capacity(){
            if Self::valid(&current){
                let s:String = current.iter().collect();
                result.push(s);
            }
        }else{
            // 可参照 二叉树 遍历来理解，这里是DFS遍历，左为'('，右为')'，递归方式遍历
            current[pos] = '(';
            Self::generate_all(current, pos+1, result);
            current[pos] = ')';
            Self::generate_all(current, pos+1, result);
        }
    }

    pub fn valid(current: &Vec<char>) -> bool {
        let mut balance = 0;
        for &c in current.iter(){
            if c == '(' {
                balance += 1;
            }else{
                balance -= 1;
            }
            if balance < 0 {
                return false;
            }
        }
        return balance == 0
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut combinations:Vec<String> = Vec::new();
        let mut current:Vec<char> = vec!['0'; 2 * n as usize];//Vec::with_capacity(2 * n as usize);
        Self::generate_all(&mut current, 0, &mut combinations);
        return combinations;
    }
}

fn main() {
    println!(
        "result:{:?}\n",
        Solution::generate_parenthesis(3));

}
