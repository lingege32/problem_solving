struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            // non-negative => 0 is allowed
            return 0;
        }
        if x < 4 {
            return 1;
        }
        let mut lower = 1;
        let mut higher = 46340.min(x / 2); // max possible root sqrt(i32::MAX)

        // Boundary conditions for optimization

        if x >= higher * higher {
            return higher;
        }

        while lower < higher {
            let mid = (lower + higher) / 2;
            if mid * mid < x {
                lower = mid + 1;
            } else {
                higher = mid;
            }
        }

        if lower * lower == x {
            lower
        } else {
            lower - 1
        }
    }
}
