/*
 * 最接近的三数之和
 * https://leetcode-cn.com/problems/3sum-closest/
 */

struct Solution {}

impl Solution {

    // 双指针
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut result:Option<i32> = None;
        if nums.len() <= 3 {
            return nums.iter().sum();
        }

        let mut nums = nums.clone();
        nums.sort();
        for i in 0..nums.len(){
            let x:i32 = nums[i];

            //相同值，可能导致重复结果
            if i > 0 && nums[i-1] == nums[i] {
                continue;
            }

            // 左指针
            let mut lp = i+1;
            let mut rp = nums.len()-1;

            // 右指针位置大于左指针
            while rp > lp {
                //相同值，可能导致重复结果
                if lp > i+1 && nums[lp]==nums[lp-1]{
                    lp += 1;
                    continue;
                }

                let y:i32 = nums[lp];
                let z:i32 = nums[rp];

                let rs = x + y + z;

                // 相等则立即返回
                if rs == target {
                    return target;
                }

                if result == None{
                    result = Some(rs);
                }

                // 更新结果值
                if (rs-target).abs() < (result.unwrap()-target).abs(){
                    //println!("{:?}",(x,y,z));
                    result = Some(rs);
                }

                if rs > target{
                    rp -= 1;
                } else {
                    lp += 1;
                }
            }
        }
        return result.unwrap();
    }

}

fn main() {
    println!(
        "{:?}",
        Solution::three_sum_closest(vec![-1,2,1,-4],1)
    );

    println!(
        "{:?}",
        Solution::three_sum_closest(vec![0,2,1,-3],1)
    );


}
