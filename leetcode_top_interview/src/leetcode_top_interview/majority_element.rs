struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        let mut majority_element = 0;

        for i in nums {
            if counter == 0 {
                majority_element = i;
            }

            if i == majority_element {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
        return majority_element;
    }
}
