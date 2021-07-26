struct Solution {}
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut left = 0;
        let mut right = matrix.len() - 1;
        while left < right {
            Self::rotate_level(matrix, left, right, matrix.len());
            left += 1;
            right -= 1;
        }
    }
    fn rotate_level(matrix: &mut Vec<Vec<i32>>, left: usize, right: usize, len: usize) {
        let mut top = (left, left);
        let mut ri = (left, right);
        let mut bottom = (right, right);
        let mut le = (right, left);
        for _ in left..right {
            unsafe {
                std::ptr::swap_nonoverlapping(
                    &mut matrix[top.0][top.1],
                    &mut matrix[ri.0][ri.1],
                    1,
                );
                std::ptr::swap_nonoverlapping(
                    &mut matrix[top.0][top.1],
                    &mut matrix[bottom.0][bottom.1],
                    1,
                );
                std::ptr::swap_nonoverlapping(
                    &mut matrix[top.0][top.1],
                    &mut matrix[le.0][le.1],
                    1,
                );
                top.1 += 1;
                ri.0 += 1;
                bottom.1 -= 1;
                le.0 -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut left = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let right = [[7, 4, 1], [8, 5, 2], [9, 6, 3]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        Solution::rotate(&mut left);
        assert_eq!(left, right);
    }
}
