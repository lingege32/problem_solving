struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![0i32; temperatures.len()];
        let mut hottest = *temperatures.last().unwrap();
        for idx in (0..ret.len() - 1).rev() {
            let mut cmp_idx = idx + 1;
            let cur = temperatures[idx];
            if cur >= hottest {
                hottest = cur;
                continue;
            }
            loop {
                let cmp = temperatures[cmp_idx];
                if cmp > cur {
                    ret[idx] = (cmp_idx - idx) as i32;
                    break;
                }
                cmp_idx += ret[cmp_idx] as usize;
            }
        }
        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 1, 1, 0],
            Solution::daily_temperatures(vec![30, 40, 50, 60])
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            vec![1, 1, 0],
            Solution::daily_temperatures(vec![30, 60, 90])
        );
    }
}
