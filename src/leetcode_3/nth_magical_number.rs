struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if a > b {
                gcd(b, a)
            } else if a == 0 {
                b
            } else {
                gcd(b % a, a)
            }
        }
        let lcm = a * b / gcd(a, b); //least common multiple
        let mut v = vec![];
        for i in (a..=lcm).step_by(a as usize) {
            v.push(i);
        }
        for i in (b..lcm).step_by(b as usize) {
            v.push(i);
        }
        v.sort_unstable();
        v.dedup();
        let nth_group = (n - 1) / v.len() as i32;
        let nth_elem = (n - 1) % v.len() as i32;
        ((nth_group as u64 * lcm as u64 + v[nth_elem as usize] as u64) % (1e9 as u64 + 7)) as i32
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let (n, a, b, ans) = (1, 2, 3, 2);
        assert_eq!(ans, Solution::nth_magical_number(n, a, b));
    }
    #[test]
    fn test_2() {
        let (n, a, b, ans) = (4, 2, 3, 6);
        assert_eq!(ans, Solution::nth_magical_number(n, a, b));
    }
    #[test]
    fn test_3() {
        let (n, a, b, ans) = (5, 2, 4, 10);
        assert_eq!(ans, Solution::nth_magical_number(n, a, b));
    }
    #[test]
    fn test_4() {
        let (n, a, b, ans) = (3, 6, 4, 8);
        assert_eq!(ans, Solution::nth_magical_number(n, a, b));
    }

    #[test]
    fn test_5() {
        let (n, a, b, ans) = (1000000000, 40000, 40000, 999720007);
        assert_eq!(ans, Solution::nth_magical_number(n, a, b));
    }
}
