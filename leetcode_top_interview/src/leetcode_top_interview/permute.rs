struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let total = (1..=nums.len()).product::<usize>();
        (0..total)
            .map(|_| {
                let clone = nums.clone();
                Self::next_permute(&mut nums);
                clone
            })
            .collect()
    }
    fn next_permute(nums: &mut Vec<i32>) {
        match nums
            .windows(2)
            .enumerate()
            .rev()
            .find(|(_, window)| window[0] < window[1])
            .map(|(idx, _)| idx)
        {
            Some(switch_idx) => {
                let swap_idx =
                    switch_idx + 1 + Self::find_insert(&nums[switch_idx + 1..], nums[switch_idx]);
                nums.swap(switch_idx, swap_idx);
                nums[switch_idx + 1..].reverse();
            }
            None => {
                nums.reverse();
            }
        }
    }

    fn find_insert(nums: &[i32], target: i32) -> usize {
        let (mut left, mut right) = (0, nums.len());
        while left + 1 < right {
            let mid = (left + right) / 2;
            if nums[mid] > target {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let v = vec![1, 2, 3, 4, 5];
        println!("{:?}", Solution::permute(v));
    }
}
