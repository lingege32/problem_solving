struct Solution();
use std::collections::HashSet;
impl Solution {
    #[allow(dead_code)]
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut ans = 1;
        let mut n = 1;
        let mut v = HashSet::<i32>::new();
        loop {
            n = n % k;
            if n == 0 {
                return ans;
            }
            if v.contains(&n) {
                break;
            } else {
                v.insert(n);
            }
            n = n*10+1;
            ans+=1;
        }
        -1
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::smallest_repunit_div_by_k(1));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(5));
    }

    #[test]
    fn test_4() {
        assert_eq!(3, Solution::smallest_repunit_div_by_k(3));
    }

    #[test]
    fn test_5() {
        assert_eq!(6, Solution::smallest_repunit_div_by_k(7));
    }
}