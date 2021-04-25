
struct Solution{}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let total = m+n;

        if total <= 2 {
            return ((nums1.iter().sum::<i32>()+nums2.iter().sum::<i32>()) as f64) / (total as f64)
        }
        let max_idx = total / 2;

        //偶数
        let mut is_odd = false;

        if total % 2 == 0 {
            is_odd = true;
        }

        println!("total:{:?},max_idx:{:?}",total,max_idx);

        let mut count = 0;
        let mut m_idx = 0;
        let mut n_idx = 0;
        let mut last_val = 0;
        let mut current_val = 0;
        while count <= max_idx{
            last_val = current_val;
            count = count + 1;
            if m_idx < m && n_idx < n{
                //两者都有元素
                let mval = nums1[m_idx];
                let nval = nums2[n_idx];
                if mval < nval {
                    println!("mval:{:?}",mval);
                    m_idx = m_idx + 1;
                    current_val = mval;
                }else{
                    println!("nval:{:?}",nval);
                    n_idx = n_idx + 1;
                    current_val = nval;
                }
                //continue;
            } else if m_idx < m && n_idx >= n{
                //只有nums1有元素
                println!("smval:{:?}",nums1[m_idx]);
                current_val = nums1[m_idx];
                m_idx += 1;
                //continue;
            } else if n_idx < n && m_idx >= m{
                //只有nums2有元素
                println!("snval:{:?}",nums2[n_idx]);
                current_val = nums2[n_idx];
                n_idx = n_idx + 1;
                //continue;
            }else {
                println!("error index");
            }
        }

        //println!("{:?},{:?},{:?}",last_val, current_val,count);
        if is_odd {
            ((last_val + current_val) as f64)/2f64
        } else {
            current_val as f64
        }
    }
}

fn main() {
    println!("{:?}",Solution::find_median_sorted_arrays(vec![1,2],vec![3,4]));
    println!("{:?}",Solution::find_median_sorted_arrays(vec![1,3],vec![2]));
    println!("{:?}",Solution::find_median_sorted_arrays(vec![0,0],vec![0,0]));
    println!("{:?}",Solution::find_median_sorted_arrays(vec![1],vec![]));
    println!("{:?}",Solution::find_median_sorted_arrays(vec![1],vec![0]));
    println!("{:?}",Solution::find_median_sorted_arrays(vec![2],vec![]));
}
