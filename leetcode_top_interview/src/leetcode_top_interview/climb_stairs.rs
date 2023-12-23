struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn climb_stairs(n: i32) -> i32 {
        let mut prev = 1;
        let mut current = 1;

        for _ in 2..=n {
            let tmp = current;
            current += prev;
            prev = tmp;
        }

        current
    }
}
