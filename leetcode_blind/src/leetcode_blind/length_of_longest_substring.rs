struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let v = s.chars().map(|c| c as u8).collect::<Vec<_>>();
        let mut dic = [usize::MAX; 256];
        let mut left = 0;
        let mut ret = 0;
        for (idx, val) in v.iter().enumerate() {
            if dic[*val as usize] != usize::MAX {
                loop {
                    let val_left = v[left];
                    dic[val_left as usize] = usize::MAX;
                    left += 1;
                    if val_left == *val {
                        break;
                    }
                }
            }
            dic[*val as usize] = idx;
            ret = ret.max((idx - left) as i32 + 1);
        }
        ret
    }
}
