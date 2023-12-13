struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left_max, mut right_max) = (0, 0);
        let (mut left, mut right) = (0usize, height.len() - 1);
        let mut total = 0;
        while left < right {
            if height[left] <= height[right] {
                if left_max > height[left] {
                    total += left_max - height[left];
                } else {
                    left_max = height[left];
                }
                left += 1;
            } else {
                if right_max > height[right] {
                    total += right_max - height[right];
                } else {
                    right_max = height[right];
                }
                right -= 1;
            }
        }

        total
    }
}
