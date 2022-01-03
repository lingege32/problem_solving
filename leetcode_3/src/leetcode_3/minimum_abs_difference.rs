struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut arr = arr;
        arr.sort_unstable();
        let min = arr.windows(2).map(|x| x[1] - x[0]).min().unwrap();
        for window in arr.windows(2) {
            if window[1] - window[0] == min {
                ans.push(window.to_vec());
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
        let arr = vec![4, 2, 1, 3];
        let ans = [[1, 2], [2, 3], [3, 4]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(Solution::minimum_abs_difference(arr), ans);
    }

    #[test]
    fn test_12() {
        let arr = vec![1, 3, 6, 10, 15];
        let ans = [[1, 3]].iter().map(|x| x.to_vec()).collect::<Vec<_>>();
        assert_eq!(Solution::minimum_abs_difference(arr), ans);
    }

    #[test]
    fn test_13() {
        let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let ans = [[-14, -10], [19, 23], [23, 27]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(Solution::minimum_abs_difference(arr), ans);
    }
}
