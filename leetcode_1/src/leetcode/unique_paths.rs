struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            1
        } else {
            let m = m as i64;
            let n = n as i64;
            let (m, n) = if m < n {
                (m - 1 , m + n - 2 ) 
            } else {
                (n - 1, m + n - 2) 
            };
            println!("C{}_{}",n,m);
            ((n - m + 1..=n).fold(1, |acc, x| x * acc) / (1..=m).fold(1, |acc, x| x * acc) ) as i32
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(28, Solution::unique_paths(3, 7));
    }
    #[test]
    fn test_2() {
        assert_eq!(6, Solution::unique_paths(3, 3));
    }
    #[test]
    fn test_3() {
        assert_eq!(48620, Solution::unique_paths(10, 10));
    }
}
