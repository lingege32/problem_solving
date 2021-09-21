struct Solution {
    
}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // full answer for n < 45
        // let full_answer = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903];
        // return full_answer[n as usize-1];


        // dp method
        let mut dp = [None; 46];
        dp[1] = Some(1);
        dp[2] = Some(2);
        dp[3] = Some(3);
        fn climb_stairs_dp(n: i32, dp: &mut [Option<i32>]) -> i32{
            match dp[n as usize] {
                Some(i) => {
                    i
                },
                None => {
                    let (l,r) = (climb_stairs_dp(n-2, dp), climb_stairs_dp(n-1, dp));
                    dp[n as usize] = Some(l+r);
                    l+r
                },
            }
        }

        climb_stairs_dp(n, &mut dp)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }
    #[test]
    fn test_2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
    #[test]
    fn test_3() {
        assert_eq!(45, Solution::climb_stairs(45));
    }
}