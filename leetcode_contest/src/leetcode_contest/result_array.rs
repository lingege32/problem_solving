struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {

        let len = nums.len();
        let mut arr1 = Vec::with_capacity((len+1)/2);
        let mut arr2 = Vec::with_capacity((len+1)/2);
        arr1.push(nums[0]);
        arr2.push(nums[1]);
        for i in 2..len {
            if arr1.last().unwrap() > arr2.last().unwrap() {
                arr1.push(nums[i]);
            } else {
                arr2.push(nums[i]);
            }
        }
        arr1.append(&mut arr2);
        arr1
    }
}