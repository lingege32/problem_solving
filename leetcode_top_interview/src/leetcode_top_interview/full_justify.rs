struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut cur_left = 0usize;
        let mut cur_len = 0;
        let mut ret = vec![];
        for (idx, len) in words.iter().map(|x| x.len()).enumerate() {
            cur_len += len;
            if cur_len > max_width {
                ret.push(Self::join_with(
                    &words[cur_left..idx],
                    cur_len - len - (idx - cur_left),
                    max_width,
                ));
                cur_len = len + 1;
                cur_left = idx;
            } else {
                cur_len += 1;
            }
        }

        let s = words[cur_left..].join(" ");
        if !s.is_empty() {
            ret.push(format!("{}{}", s, Self::get_space(max_width - s.len())))
        }

        ret
    }

    fn join_with(words: &[String], total_len: usize, max_width: usize) -> String {
        let len = words.len() - 1;
        if len == 0 {
            let s = &words[0];
            return format!("{}{}", s, Self::get_space(max_width - s.len()));
        }
        let total_space = max_width - total_len;
        let less_space = total_space / len;
        let small_space = Self::get_space(less_space);
        let big_space = Self::get_space(less_space + 1);
        let num_of_big = total_space % len;
        let mid = num_of_big + 1;
        format!(
            "{}{}{}",
            words[0..mid].join(&big_space),
            small_space,
            words[mid..].join(&small_space)
        )
    }

    fn get_space(size: usize) -> String {
        " ".repeat(size)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let words = [
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
        let ans = ["This    is    an", "example  of text", "justification.  "]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        assert_eq!(ans, Solution::full_justify(words, 16));
    }
}
