/*
 * 盛最多水的容器
 * https://leetcode-cn.com/problems/container-with-most-water/
*/


struct Solution {}

impl Solution {
    pub fn _max_area1(height: Vec<i32>) -> i32 {
        let mut max:i32 = 0;
        //let mut prev_idx = 0;
        //let mut prev_
        for (i_idx,i_value) in height.iter().enumerate(){
            for(j_idx,j_value) in height.iter().enumerate(){
                if i_idx == j_idx{
                    continue;
                }

                let w = ((j_idx as i32 - i_idx as i32)).abs();
                let mut h = i_value;
                if j_value < i_value{
                    h = j_value;
                }
                let result = w*h;
                if result > max {
                    max = w*h;
                    //println!("{:?},{:?}",i_value,j_value)
                }
            }
        }
        return max;
    }

    // 双指针法
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = height.len();
        let mut i = 0;
        let mut j = len-1;


        while i < j {
            let x: i32 = height[i]; //height.get(i).unwrap();
            let y: i32 = height[j]; //height.get(j).unwrap();
            let w = (j - i) as i32;
            let h = if x < y {
                i += 1;
                x
            } else {
                j -= 1;
                y
            };
            let area = w * h;
            if result < area {
                result = area
            };
        }

        return result;
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::max_area(vec![1,3,4,5,6,7])
    );
    println!(
        "{:?}",
        Solution::max_area(vec![1,8,6,2,5,4,8,3,7])
    );
}
