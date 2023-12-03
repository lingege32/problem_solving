struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        if mountain.len() <= 2 {
            return vec![];
        }
        let mut ret = vec![];
        for (idx, window) in mountain.windows(3).enumerate() {
            if window[1] > window[0] && window[1] > window[2] {
                ret.push(idx as i32 + 1);
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mountain = vec![2, 4, 4];
        let ans = Vec::<i32>::new();
        assert_eq!(ans, Solution::find_peaks(mountain));
    }
    #[test]
    fn test_2() {
        let mountain = vec![1, 4, 3, 8, 5];
        let ans = vec![1, 3];
        assert_eq!(ans, Solution::find_peaks(mountain));
    }
}
