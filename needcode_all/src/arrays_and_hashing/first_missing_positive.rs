struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut min = nums.len();
        for i in (0..len).rev() {
            let item = nums[i] as usize;
            if item <= 0 || item >len {
                min = min.min(i);
                nums[i] = 0;
            } else {
                let mut cur = i;
                let mut tmp = item;
                nums[i] = 0;
                while tmp > 0 && tmp <= len && tmp != cur+1 {
                    cur = tmp-1;
                    tmp = nums[cur] as usize;
                    nums[cur] = (cur+1) as i32;
                }
                if cur!=i {
                    min = min.min(i);
                }
            }
        }
        (min+1) as i32
    }
}