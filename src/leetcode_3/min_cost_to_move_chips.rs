struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut even, mut odd) = (0, 0);
        for i in position {
            if i % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        if odd > even {
            even
        } else {
            odd
        }
    }
}
