/*
正则表达式
https://leetcode-cn.com/problems/regular-expression-matching/
 */

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.chars().count(); // 字符串s长度
        let n = p.chars().count(); // 表达式p长度
        let mut f = vec![vec![false;n+1]; m+1]; // 状态 dp
        f[0][0] = true;                         // 边界条件，即两个空字符串可匹配
        for i in 0 ..= m {
            for j in 1 ..= n {
                let pnc = p.chars().nth(j-1).unwrap();
                if pnc =='*' {                         // 表达式字符为*时
                    f[i][j] = f[i][j-2];               // 匹配0个字符情况，丢掉表达式的星号和前一字符
                    if Self::matches(&s,&p,i,j-1) { // 对比与表达式的前一字符是否相同
                        f[i][j] = f[i][j]||f[i-1][j]; // 匹配多个(i-1)，则丢掉字符串的一个字符再继续匹配
                    }
                }else{          // 表达式字符为.或其他时dp[i][j] = dp[i-1][j-1]
                    if Self::matches(&s,&p,i,j){
                        f[i][j] = f[i-1][j-1];
                    }
                }
                // let pn = p.chars().nth(j-1);
                // match pn {
                //     Some('*') =>{
                //         f[i][j] = f[i][j-2];
                //         if Self::matches(&s,&p,i,j-1) {
                //             f[i][j] = f[i][j]||f[i-1][j];
                //         }
                //     },
                //     _ => {
                //         if Self::matches(&s,&p,i,j){
                //             println!("{:?},{:?},{:?},{:?}",i,j,m,n);
                //             f[i][j] = f[i-1][j-1];
                //         }
                //     }
                // }
            }
        }
        println!("{:?}",f);
        return f[m][n];
    }

    pub fn matches(s: &String, p: &String, i: usize, j: usize) -> bool{
        if i == 0 {             // 
            return false;
        }

        let pnc = p.chars().nth(j-1).unwrap();
        if pnc == '.'{          // 点匹配任意字符
            return true;
        }
        let snc = s.chars().nth(i-1).unwrap();
        return snc==pnc;        // 相等性匹配

        // let pn = p.chars().nth(j-1);
        // if let Some(pnc) = pn {
        //     if pnc == '.'{
        //         return true;
        //     }
        // }
        // let sn = s.chars().nth(i-1);
        // if let Some(pnc) = pn  {
        //     if let Some(snc) = sn  {
        //         return pn == sn;
        //     }
        // }
        // panic!("error")
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::is_match(String::from("hello"), String::from("hello"))
    );
    println!(
        "{:?}",
        Solution::is_match(String::from("hello"), String::from("hel*"))
    );
    println!(
        "{:?}",
        Solution::is_match(String::from("hello"), String::from("hel.*"))
    )
}
