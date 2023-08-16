struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut best = 0;
        let mut i = 0;
        let mut j = height.len() - 1;

        while i < j {
            let attempt = (j - i) as i32 * height[i].min(height[j]);
            best = best.max(attempt);

            if height[i] < height[j] {
                unsafe {
                    let mut next_i = i + 1;
                    while next_i < j && height.get_unchecked(next_i) <= height.get_unchecked(i) {
                        next_i += 1;
                    }
                    i = next_i;
                }
            } else {
                unsafe {
                    let mut next_j = j - 1;
                    while i < next_j && height.get_unchecked(next_j) <= height.get_unchecked(j) {
                        next_j -= 1;
                    }
                    j = next_j;
                }
            }
        }

        return best;
    }
}
