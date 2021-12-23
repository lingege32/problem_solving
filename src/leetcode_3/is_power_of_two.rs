struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn is_power_of_two(n: i32) -> bool {
        if n > 0 {
            (n & n - 1) == 0
        } else {
            false
        }
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let n = 1;
        let ans = true;
        assert_eq!(ans, Solution::is_power_of_two(n));
    }
    #[test]
    fn test_2() {
        let n = 16;
        let ans = true;
        assert_eq!(ans, Solution::is_power_of_two(n));
    }
    #[test]
    fn test_3() {
        let n = 3;
        let ans = false;
        assert_eq!(ans, Solution::is_power_of_two(n));
    }
}
