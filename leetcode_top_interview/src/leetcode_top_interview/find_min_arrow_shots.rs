struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|interval| interval[1]);

        let mut arrows = 1;
        let mut right = points[0][1];

        for interval in points.into_iter().skip(1) {
            if interval[0] > right {
                arrows += 1;
                right = interval[1];
            }
        }

        arrows
    }
}
