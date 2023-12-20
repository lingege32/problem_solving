struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut v = vec![0; m.max(n)];
        for (x, row) in matrix.iter().enumerate() {
            for (y, &val) in row.iter().enumerate() {
                if val == 0 {
                    v[x] |= 0x1;
                    v[y] |= 0x2;
                }
            }
        }
        for (x, row) in matrix.iter_mut().enumerate() {
            for (y, val) in row.iter_mut().enumerate() {
                if v[x] & 0x1 != 0 || v[y] & 0x2 != 0 {
                    *val = 0;
                }
            }
        }
    }
}
