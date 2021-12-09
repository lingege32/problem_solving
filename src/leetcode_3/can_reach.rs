struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited = vec![false; arr.len()];
        let mut stack = vec![start];
        while let Some(top) = stack.pop() {
            let offset = arr[top as usize];
            if offset == 0 {
                return true;
            }
            visited[top as usize] = true;
            if offset <= top && !visited[(top - offset) as usize] {
                stack.push(top - offset);
            }
            let right = (top + offset) as usize;
            if right < arr.len() && !visited[right] {
                stack.push(right as i32);
            }
        }
        false
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 5;
        assert!(Solution::can_reach(arr, start));
    }

    #[test]
    fn test_2() {
        let arr = vec![3, 0, 2, 1, 2];
        let start = 2;
        assert!(!Solution::can_reach(arr, start));
    }

    #[test]
    fn test_3() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 0;
        assert!(Solution::can_reach(arr, start));
    }
}
