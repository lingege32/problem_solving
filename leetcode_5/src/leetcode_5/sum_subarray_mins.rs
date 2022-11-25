struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut dp = vec![0; arr.len()];
        let mut stack = Vec::new();
        let mut sum =0 ;
        for idx in 0..arr.len() {
            let v = *unsafe{arr.get_unchecked(idx)};
            while !stack.is_empty() && *unsafe{arr.get_unchecked(*stack.last().unwrap())} >= v {
                stack.pop();
            }
            if stack.is_empty() {
                dp[idx] = (idx as i32 + 1 ) * v; 
            } else {
                let prev = *stack.last().unwrap();
                dp[idx] = dp[prev] + (idx-prev) as i32 * v;
            }
            sum = (sum + dp[idx]) % (1_000_000_007);
            stack.push(idx);
        }
        sum
    }
}


#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let arr = vec![11,81,94,43,3];
        assert_eq!(444, Solution::sum_subarray_mins(arr));
    }
}