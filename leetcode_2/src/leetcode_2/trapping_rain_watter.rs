use itertools::merge;
/* Other's Solution
use std::cmp::max;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let height = height.iter().map(|x| *x as usize).collect::<Vec<usize>>();
        if height.len() == 0 { return 0 as i32; }

        let mut left = 0;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut right = height.len() - 1;
        let mut water = 0;

        while left != right {
            left_max = max(height[left], left_max);
            right_max = max(height[right], right_max);

            if left_max < right_max {
                water += left_max - height[left];
                left += 1;
            } else {
                water += right_max - height[right];
                right -= 1;
            }
        }
        water as i32
    }
}
 */
struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
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
        // println!("{:?}", &height[begin..=end]);
        fn inner_trap(height: &[i32]) -> i32 {
            let mut left = 0usize;
            let mut tran = false;
            let mut vec = Vec::new();
            let mut merge_to_vec_back = |mut left: usize, right: usize| loop {
                match vec.pop() {
                    Some((old_left, old_right)) => {
                        if height[old_right] <= height[old_left]
                            && height[old_right] <= height[right]
                        {
                            left = old_left;
                        } else {
                            vec.push((old_left, old_right));
                            vec.push((left, right));
                            break;
                        }
                    }
                    None => {
                        vec.push((left, right));
                        break;
                    }
                }
            };
            for idx in 1..height.len() {
                if !tran && height[idx] > height[idx - 1] {
                    tran = true;
                } else if tran && height[idx] < height[idx - 1] {
                    tran = false;
                    merge_to_vec_back(left, idx - 1);
                    left = idx - 1;
                }
            }
            let idx = height.len();
            merge_to_vec_back(left, idx - 1);
            let mut ans = 0;
            // println!("vec: {:?}", vec);
            for (left, right) in vec {
                let min = height[left].min(height[right]);
                for idx in (left + 1)..right {
                    let add = min - height[idx];
                    if add > 0 {
                        ans += add;
                    }
                }
            }
            ans
        }

        inner_trap(&height[begin..=end])
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(6, Solution::trap(height.to_vec()));
    }
    #[test]
    fn test_2() {
        let height = [4, 2, 0, 3, 2, 5];
        assert_eq!(9, Solution::trap(height.to_vec()));
    }
    #[test]
    fn test_3() {
        let height = [2, 0, 2];
        assert_eq!(2, Solution::trap(height.to_vec()));
    }
    #[test]
    fn test_4() {
        let height = [5, 4, 1, 2];
        assert_eq!(1, Solution::trap(height.to_vec()));
    }
    #[test]
    fn test_5() {
        let height = [4, 2, 0, 3, 2, 4, 3, 4];
        assert_eq!(10, Solution::trap(height.to_vec()));
    }
    #[test]
    fn test_6() {
        let height = [
            6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1, 2, 1, 3, 4, 6, 8, 1, 3,
        ];
        assert_eq!(83, Solution::trap(height.to_vec()));
    }
}
