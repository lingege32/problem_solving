struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut sum = vec![0; triangle.len()];
        sum[0] = triangle[0][0];
        for column in triangle.iter().skip(1) {
            let item = column.len() - 1;
            sum[item] = sum[item - 1] + column[item];
            for idx in (1..item).rev() {
                let m = sum[idx].min(sum[idx - 1]);
                sum[idx] = m + column[idx];
            }
            sum[0] = sum[0] + column[0];
        }
        *sum.iter().min().unwrap()
    }
}
