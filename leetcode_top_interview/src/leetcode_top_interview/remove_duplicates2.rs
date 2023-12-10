struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 3 {
            return len as i32;
        }
        let mut read = 2;
        let mut write = 2;
        while read != len {
            let r = nums[read];
            let w = nums[write - 2];
            if r != w {
                nums[write] = r;
                write += 1;
            }
            read += 1;
        }
        write as i32
    }
}
