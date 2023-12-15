struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut ret = 0;
        while left < right {
            let l = height[left];
            let r = height[right];
            let water = l.min(r) * (right - left) as i32;
            ret = ret.max(water);
            if l < r {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ret
    }
}
