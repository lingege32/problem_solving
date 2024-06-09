struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut y_arr = [0, 0, 0];
        let mut non_y_arr = [0, 0, 0];
        for rows in grid.iter() {
            for element in rows {
                non_y_arr[*element as usize] += 1;
            }
        }

        let m = n / 2;
        let mut row_id = 0;
        for i in 0..m {
            let l = grid[row_id][i];
            let r = grid[row_id][n - 1 - i];
            non_y_arr[l as usize] -= 1;
            y_arr[l as usize] += 1;
            non_y_arr[r as usize] -= 1;
            y_arr[r as usize] += 1;
            row_id += 1;
        }
        for rid in m..n {
            let e = grid[rid][m];
            non_y_arr[e as usize] -= 1;
            y_arr[e as usize] += 1;
        }
        // println!("{:?}", non_y_arr);
        // println!("{:?}", y_arr);
        let mut ans = i32::MAX;
        for y in 0..3 {
            let mut y_changed = 0;
            for i in 0..3 {
                if i != y {
                    y_changed += y_arr[i as usize];
                }
            }
            for non_y in 0..3 {
                if non_y == y {
                    continue;
                }
                let mut non_y_changed = 0;
                for i in 0..3 {
                    if i != non_y {
                        non_y_changed += non_y_arr[i as usize];
                    }
                }
                ans = ans.min(y_changed + non_y_changed);
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
        let grid = [[1, 2, 2], [1, 1, 0], [0, 1, 0]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(3, Solution::minimum_operations_to_write_y(grid))
    }
}
