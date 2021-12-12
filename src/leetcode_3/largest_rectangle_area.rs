struct Solution {}
// 12ms faster than 80%
impl Solution {
    #[allow(dead_code)]
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut largest = 0;

        let mut i = 0;
        while i < heights.len() {
            if stack.is_empty() || heights[*stack.last().unwrap()] <= heights[i] {
                stack.push(i);
                i += 1;
                continue;
            }
            let current = stack.pop().unwrap();
            let width = match stack.is_empty() {
                true => i as i32,
                _ => i as i32 - *stack.last().unwrap() as i32 - 1,
            };
            let area = heights[current] * width;
            largest = largest.max(area);
        }
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            let width = match stack.is_empty() {
                true => i as i32,
                _ => i as i32 - *stack.last().unwrap() as i32 - 1,
            };
            let area = heights[current] * width;
            largest = largest.max(area);
        }
        largest
    }
}

// 20ms faster than 40%
// impl Solution {
//     #[allow(dead_code)]
//     pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//         fn inner(heights: &[i32]) -> Vec<i32> {
//             let mut a = (0..heights.len()).collect::<Vec<usize>>();
//             let mut monotonic_stack = Vec::new();
//             for idx in 0..heights.len() {
//                 let cur_height = heights[idx];
//                 while !monotonic_stack.is_empty()
//                     && cur_height <= heights[*monotonic_stack.last().unwrap()]
//                 {
//                     let left = monotonic_stack.pop().unwrap();
//                     a[idx] = a[left];
//                 }
//                 monotonic_stack.push(idx);
//             }
//             // println!("heights: {:?}, a: {:?}", heights, a);
//             let a = a
//                 .into_iter()
//                 .enumerate()
//                 .map(|(cur, left)| (cur - left + 1) as i32 * heights[cur])
//                 .collect::<Vec<i32>>();
//             a
//         }
//         let mut heights = heights;
//         let mut left = inner(&heights);
//         heights.reverse();
//         let right = inner(&heights);
//         left.reverse();
//         let mut max = *heights.iter().max().unwrap();
//         for idx in 0..heights.len() {
//             max = max.max(left[idx] + right[idx] - heights[idx]);
//         }
//         max
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_5v() {
        let heights = vec![1, 2, 4, 4, 5, 3, 3];
        let ans = 15;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }

    #[test]
    fn test_m0() {
        let heights = vec![2, 1];
        let ans = 2;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }
    #[test]
    fn test_0() {
        let heights = vec![2, 1, 2];
        let ans = 3;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }
    #[test]
    fn test_1() {
        let heights = vec![2, 1, 5, 6, 2, 3];
        let ans = 10;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }

    #[test]
    fn test_2() {
        let heights = vec![2, 4];
        let ans = 4;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }

    #[test]
    fn test_3() {
        let heights = vec![1, 1];
        let ans = 2;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }

    #[test]
    fn test_4() {
        let heights = vec![1, 2, 2];
        let ans = 4;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }

    #[test]
    fn test_5() {
        let heights = vec![1, 2, 3, 4, 5];
        let ans = 9;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }

    #[test]
    fn test_6() {
        let heights = vec![4, 2, 0, 3, 2, 5];
        let ans = 6;
        assert_eq!(ans, Solution::largest_rectangle_area(heights));
    }
}
