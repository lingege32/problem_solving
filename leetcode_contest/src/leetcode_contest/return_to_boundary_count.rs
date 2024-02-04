struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut c = 0;
        let mut ret = 0;
        for n in nums {
            c += n;
            if c == 0 {
                ret += 1;
            }
        }
        ret
    }
}
