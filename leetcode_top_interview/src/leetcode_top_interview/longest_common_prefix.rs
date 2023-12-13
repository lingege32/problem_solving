struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_len = strs.iter().map(|x| x.len()).min().unwrap();
        let mut another_strs = strs.iter().map(|x| x.as_bytes().iter()).collect::<Vec<_>>();
        let mut idx = 0;
        'out: for _ in 0..min_len {
            let c = another_strs[0].next().unwrap();
            for other in another_strs[1..].iter_mut() {
                let another_c = other.next().unwrap();
                if *c != *another_c {
                    break 'out;
                }
            }
            idx += 1;
        }

        strs[0][0..idx].to_string()
    }
}
