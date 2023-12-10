struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by(|&l, &r| r.cmp(&l));
        let mut h_index = 0;
        for (num, citied) in citations.into_iter().enumerate() {
            let n = num as i32 + 1;
            if h_index >= citied {
                break;
            }
            let m = n.min(citied);
            h_index = h_index.max(m);
        }
        h_index
    }
}
