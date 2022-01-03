struct Solution {}
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn next_unique_permutation(nums: &mut [i32]) -> bool {
            match (1..nums.len()).rev().find(|x| nums[*x - 1] < nums[*x]) {
                Some(idx) => {
                    let prev_idx = idx - 1;
                    let i = (idx..nums.len())
                        .rev()
                        .find(|x| nums[*x] > nums[prev_idx])
                        .unwrap();
                    nums.swap(prev_idx, i);
                    nums[idx..].reverse();

                    true
                }
                None => false,
            }
        }
        let mut nums = nums;
        nums.sort();
        let mut ans = vec![nums.clone()];
        while next_unique_permutation(&mut nums) {
            ans.push(nums.clone());
        }
        ans
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [1, 1, 2].to_vec();
        let mut out: Vec<Vec<i32>> = [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        let mut ans = Solution::permute_unique(nums);
        out.sort();
        ans.sort();
        assert_eq!(out, ans);
    }
}
