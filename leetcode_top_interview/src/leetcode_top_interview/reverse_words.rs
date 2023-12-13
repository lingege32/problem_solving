struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_words(s: String) -> String {
        let v = s
            .trim()
            .split(' ')
            .rev()
            .filter(|x| *x != "")
            .collect::<Vec<_>>();
        v.join(" ")
    }
}
