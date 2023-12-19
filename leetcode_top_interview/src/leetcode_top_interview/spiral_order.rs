struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row_num = matrix.len() as i32;
        if row_num == 0 {
            return vec![];
        }
        let mut column_num = matrix[0].len() as i32;
        let mut ans = Vec::<i32>::with_capacity((row_num * column_num) as usize);
        for number_loop in 0i32.. {
            if column_num<1 || row_num<1 {
                break;
            }
            let (mut x, mut y) = (number_loop as usize, number_loop as usize);
            for _ in 0..column_num - 1 {
                ans.push(matrix[x][y]);
                y += 1;
            }
            for _ in 0..row_num - 1 {
                ans.push(matrix[x][y]);
                x += 1;
            }
            if column_num != 1 && row_num != 1 {
                for _ in 0..column_num - 1 {
                    ans.push(matrix[x][y]);
                    y -= 1;
                }
                for _ in 0..row_num - 1 {
                    ans.push(matrix[x][y]);
                    x -= 1;
                }
            } else {
                ans.push(matrix[x][y]);
            }
            column_num -= 2;
            row_num -= 2;
        }
        ans
    }
}
