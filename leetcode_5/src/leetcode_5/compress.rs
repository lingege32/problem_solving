struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let len = chars.len();
        if len == 1 {
            return 1;
        }
        let mut ret = 0;
        let mut left = 0;
        while left < len {
            let c = chars[left];
            let mut right = left + 1;
            while right < len && chars[right] == c {
                right += 1;
            }
            let count = right - left;
            chars[ret] = c;
            ret += 1;

            if count != 1 {
                let count = count.to_string();
                for val in count.chars() {
                    chars[ret] = val;
                    ret += 1;
                }
            }
            left = right;
        }
        ret as i32
    }
}
