struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(2usize.pow(nums.len() as u32));
        ans.push(Vec::new());
        for l in 1..=nums.len() {
            Self::add_n(l, &mut ans, &nums);
        }
        ans
    }
    fn add_n(l: usize, ans: &mut Vec<Vec<i32>>, nums: &[i32]) {
        let mut idx_vec = (0..l).collect::<Vec<_>>();
        ans.push(idx_vec.iter().map(|x| nums[*x]).collect());
        while Self::permutate(&mut &mut idx_vec, nums.len()) {
            ans.push(idx_vec.iter().map(|x| nums[*x]).collect());
        }
    }
    fn permutate(idx_vec: &mut [usize], limit: usize) -> bool {
        let carry_bit = (0..idx_vec.len())
            .rev()
            .enumerate()
            .find(|&(enumerate, idx)| {
                1 + enumerate + idx_vec[idx] != limit
            });
        match carry_bit {
            Some((_, carry)) => {
                idx_vec[carry] += 1;
                for idx in carry + 1..idx_vec.len() {
                    idx_vec[idx] = idx_vec[idx - 1] + 1;
                }
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = [1, 2, 3].to_vec();
        let mut ans = vec![vec![],vec![1],vec![2],vec![3],vec![1,2],vec![1,3],vec![2,3],vec![1,2,3]];
        assert_eq!(Solution::subsets(nums), ans);
    }
}
