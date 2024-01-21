struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let (x, y) = if x < y { (x, y) } else { (y, x) };
        let mut ans = vec![0; n as usize];
        for row in 1..=n {
            for col in row + 1..=n {
                let dis = (col - row).min((row - x).abs() + (col - y).abs() + 1);
                ans[dis as usize - 1] += 2;
            }
        }

        ans
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let x = 1;
        let y = 3;
        println!("{:?}", Solution::count_of_pairs(n, x, y));
    }
    #[test]
    fn test_12() {
        let n = 4;
        let x = 1;
        let y = 1;
        println!("{:?}", Solution::count_of_pairs(n, x, y));
    }
    #[test]
    fn test_132() {
        let n = 5;
        let x = 2;
        let y = 4;
        println!("{:?}", Solution::count_of_pairs(n, x, y));
    }
}
