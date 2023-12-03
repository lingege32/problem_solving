struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_added_coins(mut arr: Vec<i32>, target: i32) -> i32 {
      arr.sort_unstable();
      let (mut seen_until, mut res) = (1u64, 0);
      for v in arr {
        while v as u64 > seen_until {
          seen_until += seen_until;
          res += 1;
        }
        seen_until += v as u64;
      }
  
      while seen_until < target as u64 + 1 {
        seen_until += seen_until;
        res += 1;
      }
  
      return res;
    }
  }
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let coins = vec![1, 4, 10, 5, 7, 19];
        let target = 19;
        let ans = 1;
        assert_eq!(ans, Solution::minimum_added_coins(coins, target));
    }
    #[test]
    fn test_2() {
        let coins = vec![1, 1, 1];
        let target = 20;
        let ans = 3;
        assert_eq!(ans, Solution::minimum_added_coins(coins, target));
    }
}
