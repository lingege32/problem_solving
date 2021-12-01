struct Solution {}

// BFS is quick
use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut visited = vec![false; n + 1];
        let mut queue = VecDeque::new();
        queue.push_back(n);
        let mut level = 0;

        loop {
            level += 1;

            for _ in 0..queue.len() {
                let x = queue.pop_front().unwrap();
                for i in 1.. {
                    let square = i * i;
                    if square > x {
                        break;
                    }

                    let next = x - square;
                    if next == 0 {
                        return level;
                    }

                    if !visited[next] {
                        visited[next] = true;
                        queue.push_back(next);
                    }
                }
            }
        }
    }
}

// DP is slow
// impl Solution {
//     #[allow(dead_code)]
//     pub fn num_squares(n: i32) -> i32 {
//         let mut dp = vec![i32::MAX / 2; n as usize + 1];
//         dp[0] = 0;
//         for i in 1..=n {
//             for j in (1..).map(|x| x * x).take_while(|x| *x <= i) {
//                 dp[i as usize] = (dp[(i - j) as usize] + 1).min(dp[i as usize]);
//             }
//         }
//         dp[n as usize]
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::num_squares(13));
    }
}
