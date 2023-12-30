struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::new(); // the item in dp means the smallest val for subsequence's length is idx
        dp.push(nums[0]);
        for i in nums.into_iter().skip(1) {
            // println!("i: {}, dp: {:?}", i, dp);
            if i < dp[0] {
                dp[0] = i;
            } else if i > *dp.last().unwrap() {
                dp.push(i);
            } else {
                let (mut left, mut right) = (0, dp.len());
                while left < right {
                    let mid = (left + right) / 2;
                    if dp[mid] < i {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
                dp[right] = i;
            }
        }
        dp.len() as i32
    }
}
