struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        let mut tmp = 0;
        for x in 1..len {
            for y in 0..x {
                std::mem::swap(&mut matrix[x][y], &mut tmp);
                std::mem::swap(&mut matrix[y][x], &mut tmp);
                std::mem::swap(&mut matrix[x][y], &mut tmp);
            }
        }
        matrix.iter_mut().for_each(|row| row.reverse());
    }
}
