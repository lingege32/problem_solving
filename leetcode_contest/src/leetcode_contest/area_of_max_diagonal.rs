struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        match dimensions
            .iter()
            .map(|v| {
                let x = v[0];
                let y = v[1];
                (x * x + y * y, x*y)
            })
            .max()
        {
            Some(max_idx) => max_idx.1,
            None => 0,
        }
    }
}
