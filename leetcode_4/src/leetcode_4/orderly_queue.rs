struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn orderly_queue(s: String, k: i32) -> String {
        // Solution reference: https://www.cnblogs.com/grandyang/p/10995474.html
        /* 
        Like the bubble sort
        The first two slots are the bubble and keep the big one in the slots 
        If we finish one step, put the max one to the last.

        Example 1: 5 4 3 2 1
        5 4 3 2 1
        5 3 2 1 4
        5 2 1 3 4
        5 1 2 3 4
        1 2 3 4 5

        Example 2: 1 3 5 8 2
        loop 1: 5 iterations to put the 8 to the last
        1 3 5 8 2
        3 5 8 2 1
        5 8 2 1 3
        8 2 1 3 5
        8 1 3 5 2

        loop 2: 4 iterations to put the 5 to the last
        1 3 5 2 8
        3 5 2 8 1
        5 2 8 1 3
        5 8 1 3 2
        8 1 3 2 5
        
        loop 3: 3 iterations to put the 3 to the last
        1 3 2 5 8
        3 2 5 8 1
        3 5 8 1 2
        5 8 1 2 3
        8 1 2 3 5

        1 2 3 5 8
        */
        if k > 1 {
            let mut v8 = s.into_bytes();
            v8.sort_unstable();
            return String::from_utf8(v8).unwrap();
        } else {
            let mut ret = s.clone();
            for i in 1..ret.len() {
                let mut new = s[i..].to_string();
                new.push_str(&s[0..i]);
                ret = ret.min(new);
            }
            ret
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let ans = Solution::orderly_queue("cba".to_string(), 1);
        assert_eq!("acb", ans);
    }
}
