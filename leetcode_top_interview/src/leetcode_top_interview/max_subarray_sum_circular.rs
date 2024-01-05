struct Solution;

fn kadane<T: Iterator<Item = i32>>(nums: T) -> i32 {
    let mut max = i32::MIN;
    let mut cur = 0;
    for v in nums {
        if cur < 0 {
            cur = v;
        } else {
            cur += v;
        }
        max = max.max(cur);
    }
    max
}
impl Solution {
    #[allow(dead_code)]
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let forward_max = kadane(nums.iter().copied());
        if forward_max < 0 {
            return forward_max;
        }
        let sum = nums.iter().sum::<i32>();
        let minus_max = kadane(nums.iter().map(|x| -*x));
        forward_max.max(sum + minus_max)
    }
}
