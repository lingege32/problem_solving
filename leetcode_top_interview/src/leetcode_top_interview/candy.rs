struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len = ratings.len();
        let mut ret = vec![1; len];

        for idx in 1..len {
            if ratings[idx] > ratings[idx - 1] {
                ret[idx] = ret[idx - 1] + 1;
            }
        }

        for idx in (0..len - 1).rev() {
            if ratings[idx] > ratings[idx + 1] {
                ret[idx] = ret[idx].max(ret[idx + 1] + 1);
            }
        }

        ret.into_iter().sum()
    }
}
