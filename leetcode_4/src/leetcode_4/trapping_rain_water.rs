struct Solution;
// leetcode answer GOOOD
// impl Solution {
//     pub fn trap(height: Vec<i32>) -> i32 {
//         let mut v = vec![i32::MAX; height.len()];
//         {
//             let mut max = 0;
//             for (i, &h) in height.iter().enumerate() {
//                 max = max.max(h);
//                 v[i] = v[i].min(max);
//             }
//         }
//         {
//             let mut max = 0;
//             for (i, &h) in height.iter().enumerate().rev() {
//                 max = max.max(h);
//                 v[i] = v[i].min(max);
//             }
//         }
//         height.iter().zip(&v).map(|(h, m)| m - h).sum()
//     }
// }

impl Solution {
    #[allow(dead_code)]
    pub fn trapping_rain_water(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let begin = match (1..height.len()).find(|&idx| height[idx] < height[idx - 1]) {
            Some(b) => b - 1,
            None => return 0,
        };
        let end = match (0..=height.len() - 2)
            .rev()
            .find(|&idx| height[idx] < height[idx + 1])
        {
            Some(e) => e + 1,
            None => return 0,
        };
        Solution::trap_impl(&height[begin..=end])
    }
    fn trap_impl(height: &[i32]) -> i32 {
        let mut max_of_left = 0;
        // stack is never empty
        let mut stack = vec![max_of_left];
        for (idx, value) in height.iter().enumerate().skip(1) {
            loop {
                let left_idx = stack.pop().unwrap();
                let left_val = unsafe { height.get_unchecked(left_idx) };
                if left_idx == max_of_left || value < left_val {
                    stack.push(left_idx);
                    stack.push(idx);
                    if left_val <= value {
                        max_of_left = idx;
                    }
                    break;
                }
            }
        }
        println!("{:?}\n{:?}", height, stack);
        stack
            .windows(2)
            .map(|window| {
                let (left_idx, right_idx) = (window[0], window[1]);
                let small_wall = height[left_idx].min(height[right_idx]);
                height[left_idx..right_idx]
                    .iter()
                    .map(|x| if small_wall > *x { small_wall - x } else { 0 })
                    .sum::<i32>()
            })
            .sum()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(6, Solution::trapping_rain_water(height.to_vec()));
    }
    #[test]
    fn test_2() {
        let height = [4, 2, 0, 3, 2, 5];
        assert_eq!(9, Solution::trapping_rain_water(height.to_vec()));
    }
    #[test]
    fn test_3() {
        let height = [2, 0, 2];
        assert_eq!(2, Solution::trapping_rain_water(height.to_vec()));
    }
    #[test]
    fn test_4() {
        let height = [5, 4, 1, 2];
        assert_eq!(1, Solution::trapping_rain_water(height.to_vec()));
    }
    #[test]
    fn test_5() {
        let height = [4, 2, 0, 3, 2, 4, 3, 4];
        assert_eq!(10, Solution::trapping_rain_water(height.to_vec()));
    }
    #[test]
    fn test_6() {
        let height = [
            6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1, 2, 1, 3, 4, 6, 8, 1, 3,
        ];
        assert_eq!(83, Solution::trapping_rain_water(height.to_vec()));
    }
}
