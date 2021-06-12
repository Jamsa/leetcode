/*
 * 最接近的四数之和
 * https://leetcode-cn.com/problems/4sum/
 */

struct Solution {}

impl Solution {

    // 双指针
    pub fn four_sum(nums: Vec<i32>,target: i32) -> Vec<Vec<i32>> {
        let mut result:Vec<Vec<i32>> = vec![];
        let len = nums.len();
         if len < 4 {
             return result;
         }

        let mut nums = nums.clone();
        nums.sort();
        for i in 0..(len-3){
            let x:i32 = nums[i];
            //相同值，可能导致重复结果
            if i > 0 && nums[i-1] == nums[i] {
                continue;
            }

            // 因为排过序，如果第一个数大于0，和一定不会为0
            // if x > target {
            //     break;
            // }

            //相邻的四数和大于目标值，向后取的值一定会比目标值更大，可终止循环
            if nums[i] + nums[i+1]+ nums[i+2]+ nums[i+3] > target{
                break;
            }
            //当前值与最大的三个值相加都小于目标值时，可继续
            if nums[i] + nums[len-1]+ nums[len-2]+ nums[len-3] < target{
                continue;
            }

            for j in (i+1)..(len-2) {
                let c = nums[j];
                //println!("{:?}",c);

                //相同值，可能导致重复结果
                if j > i+1 && nums[j-1] == c {
                    continue;
                }
                //相邻的四数和大于目标值，向后取的值一定会比目标值更大，可终止循环
                if nums[i] + nums[j]+ nums[j+1]+ nums[j+2] > target{
                    break;
                }
                //当前值与最大的三个值相加都小于目标值时，可继续
                if nums[i] + nums[j]+ nums[len-1]+ nums[len-2] < target{
                    continue;
                }


                // 左指针
                let mut lp = j + 1;
                let mut rp = len - 1;

                // 右指针位置大于左指针
                while rp > lp {
                    //相同值，可能导致重复结果
                    if lp > j + 1 && nums[lp] == nums[lp - 1] {
                        lp += 1;
                        continue;
                    }

                    let y: i32 = nums[lp];
                    let z: i32 = nums[rp];

                    let rs = x + y + z+c;
                    //println!("x:{:?}",vec![x,c,y,z]);
                    if rs > target {
                        rp -= 1;
                    } else if rs < target {
                        lp += 1;
                    } else {
                        let row = vec![x,c, y, z];
                        //判断非常慢，通过上面的重复元素判断来代替
                        //if !result.contains(&row){
                        result.push(row);
                        //}
                        rp -= 1;
                        lp += 1;
                    }
                }
            }
        }
        return result;
    }

}

fn main() {
    println!(
        "{:?}",
        Solution::four_sum(vec![1,0,-1,0,-2,2],0)
    );

    println!(
        "{:?}",
        Solution::four_sum(vec![],0)
    );

    println!(
        "{:?}",
        Solution::four_sum(vec![2,2,2,2,2],8)
    );

    println!(
        "{:?}",
        Solution::four_sum(vec![1,-2,-5,-4,-3,3,3,5],-11)
    );




}
