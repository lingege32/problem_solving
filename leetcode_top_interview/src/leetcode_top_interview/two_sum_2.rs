struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0usize, numbers.len() - 1);
        while left < right {
            let mix = numbers[left] + numbers[right];
            if mix == target {
                return vec![left as i32 + 1, right as i32 + 1];
            } else if mix < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        std::unreachable!()
    }
}
