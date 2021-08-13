/*
 * 有效的括号
 * https://leetcode-cn.com/problems/valid-parentheses/
 */

struct Solution {}


impl Solution {
    //不需要对称出现
    pub fn _is_valid1(s: String) -> bool {
        let mut result = vec![0,0,0];
        let rbases = vec!['{','(','[','}',')',']'];

        s.chars().for_each(|s|{
            let idx = rbases.iter().position(|&x|x==s);
            if idx != None {
                let idx:i32 = idx.unwrap() as i32;
                let value = if idx < 3 {
                    1
                } else {
                    -1
                };
                let idx = (idx % 3) as usize;
                println!("{:?},{:?},{:?}",s,idx,value);
                result[idx] += value;
            }
        });
        println!("{:?}",result);
        return result.iter().all(|&x| x == 0);
    }

    /*
    * 对称出现
    * 采用栈进行记录
    * 对所有字符进行遍历，遇到左括号字符入栈，遇到右括号出栈
    * 出栈的字符与当前遍历的字符进行对比：比较当前遍历字符(右括号)对应的左括号字符与出栈字符是否相同
    * 有不同则立即返回false
    * 遍历结束后检查栈是否为空（防止完全为左括号的形式）
    */
    pub fn is_valid(s: String) -> bool {
        let mut result = vec![];
        let rbases = vec!['{','(','[','}',')',']'];

        for i in s.chars(){
            let idx = rbases.iter().position(|&c|c==i);
            if idx != None {
                let idx:i32 = idx.unwrap() as i32;
                print!("{:?} in index :{:?},",i, idx);
                if idx < 3 {
                    //索引位小于3的符号push
                    println!("push:{:?}",i);
                    result.push(i);
                } else {
                    //大于3的符号pop
                    let p = result.pop();
                    //得到相对应的左括号字符，要求rbases有规律
                    let eqc_idx = (idx % 3) as usize;
                    let eqc = rbases[eqc_idx];

                    //相对应符号与pop的的字符是否相同
                    println!("pop:{:?} should eq {:?}",p,eqc);
                    if let Some(ps) = p{
                        if ps == eqc {
                            continue;
                        }
                    }

                    //不相同立即返回false
                    return false;
                };
            }
        }
        return result.is_empty();
    }
}

fn main() {

    println!(
        "result:{:?}\n",
        Solution::is_valid(String::from("{[}")));

    println!(
        "result:{:?}\n",
        Solution::is_valid(String::from("{{}}[]")));

    println!(
        "result:{:?}\n",
        Solution::is_valid(String::from("([)]")));

    println!(
        "result:{:?}\n",
        Solution::is_valid(String::from("[")));

}
