struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let (mut left, mut right) = (0, m * n);
        while left < right {
            let mid = (left + right) / 2;
            let val = matrix[mid / n][mid % n];
            if val == target {
                return true;
            } else if val < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        false
    }
}
