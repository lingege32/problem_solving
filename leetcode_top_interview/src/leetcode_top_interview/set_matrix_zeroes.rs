struct Solution;
use std::collections::HashSet;
impl Solution {
    #[allow(dead_code)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row_store = HashSet::<usize>::new();
        let mut column_store = HashSet::<usize>::new();
        for (x, row) in matrix.iter().enumerate() {
            for (y, &val) in row.iter().enumerate() {
                if val == 0 {
                    row_store.insert(x);
                    column_store.insert(y);
                }
            }
        }
        for row in row_store {
            for val in matrix[row].iter_mut() {
                *val = 0
            }
        }
        for col in column_store {
            for row in matrix.iter_mut() {
                row[col] = 0;
            }
        }
    }
}
