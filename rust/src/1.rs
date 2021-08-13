
struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![0,0];
        for (i, iv) in nums.iter().enumerate() {
            for (j, jv) in nums.iter().enumerate() {
                if j==i {
                    continue;
                }
                if (iv+jv) == target{
                    result[0] = i as i32;
                    result[1] = j as i32;
                    return result;
                }

            }
        }
        return result;
    }
}

fn main() {
    println!("{:?}",Solution::two_sum(vec![3,2,4],6));
}
