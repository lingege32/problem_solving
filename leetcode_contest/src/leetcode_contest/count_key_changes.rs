struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn count_key_changes(s: String) -> i32 {
        let s = s.chars().map(|x| x.to_lowercase().next().unwrap()).collect::<Vec<_>>();
        s.windows(2).filter(|w| w[0] != w[1]).count() as i32
    }
}