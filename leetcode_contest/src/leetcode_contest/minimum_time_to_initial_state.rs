struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let k = k as usize;
        let word = word.chars().collect::<Vec<_>>();
        let len = word.len();
        let mut ret = 1;
        for i in (k..len).step_by(k) {
            if Solution::check(&word, &word[i..]) {
                return ret;
            }
            ret += 1;
        }
        if len % k == 0 {
            (len / k) as i32
        } else {
            (1 + len / k) as i32
        }
    }

    fn check(s: &[char], pattern: &[char]) -> bool {
        let p_len = pattern.len();
        pattern == &s[..p_len]
    }
}
