struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut rows = Vec::new();
        for _ in 0..num_rows {
            rows.push(String::new());
        }
        let mut idx = 0;
        let mut offset = 1;
        for &item in s.as_bytes().iter() {
            //let idx = (i as i32) % num_rows;
            let opt_str: Option<&mut String> = rows.get_mut(idx as usize);
            if let Some::<&mut String>(s) = opt_str {
                s.push(item as char)
            } else {
                panic!("error index");
            }

            if idx + offset >= num_rows || idx + offset < 0 {
                offset = -1 * offset
            }
            idx += offset;
        }

        let mut result = String::new();
        for item in rows.iter() {
            result += item
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::convert(String::from("hello"), 3));
    println!("{:?}", Solution::convert(String::from("h2"), 3));
    println!("{:?}", Solution::convert(String::from("PAYPALISHIRING"), 3));
    println!("{:?}", Solution::convert(String::from("AB"), 1));
}
