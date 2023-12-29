struct Solution;
const MAPPING: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
impl Solution {
    #[allow(dead_code)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut ret = vec!["".to_string()];
        for c in digits.chars() {
            let mapping_str = MAPPING[c as usize - '2' as usize];
            let mut tmp = Vec::with_capacity(ret.len() * mapping_str.len());
            for r in ret.iter() {
                for c in mapping_str.chars() {
                    tmp.push(format!("{}{}", r, c));
                }
            }
            std::mem::swap(&mut tmp, &mut ret);
        }

        ret
    }

    /**
     * The version 2 is carry-in.
     * Check the current number and get the right character
     */
    #[allow(dead_code)]
    pub fn letter_combinations_2(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let len = digits
            .as_bytes()
            .iter()
            .map(|d| MAPPING[(d - '2' as u8) as usize].len())
            .product();

        let mut ans = vec![digits.clone(); len];

        let mut idx = 0;
        let mut m_len = len;
        for digit_idx in digits.as_bytes().iter().map(|d| (d - '2' as u8) as usize) {
            m_len /= MAPPING[digit_idx].len();
            let mut ans_idx = 0;
            while ans_idx < len {
                for c in MAPPING[digit_idx].as_bytes().iter() {
                    for _ in 0..m_len {
                        unsafe {
                            ans[ans_idx].as_mut_vec()[idx] = *c;
                        }
                        ans_idx += 1;
                    }
                }
            }
            idx += 1;
        }

        ans
    }
}
