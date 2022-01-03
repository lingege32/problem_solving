struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut v = [0; 60];
        for t in time {
            v[(t % 60) as usize] += 1;
        }
        fn cm2(val: i32) -> i32 {
            if val < 2 {
                0
            } else {
                (val) * (val - 1) / 2
            }
        }
        cm2(v[0]) + cm2(v[30]) + (1..30).map(|x| v[x] * v[60 - x]).sum::<i32>()
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let time = vec![30, 20, 150, 100, 40];
        assert_eq!(3, Solution::num_pairs_divisible_by60(time));
    }

    #[test]
    fn test_2() {
        let time = vec![60, 60, 60];
        assert_eq!(3, Solution::num_pairs_divisible_by60(time));
    }
}
