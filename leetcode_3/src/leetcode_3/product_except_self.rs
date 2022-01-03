struct Solution {}

// more efficent but more memory than the bottom 
impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        // the first for loop to calculate the nums[0] * nums[1] * ... * nums[i-1] for ret[i]
        // the second for loop to calculate ret[i] * ret[length-1] * ret[length-2] * ... * ret[i+1] for ret[i]
        let mut ret = Vec::with_capacity(length);
        let mut acc = 1;
        for num in &nums {
            ret.push(acc);
            acc *= num;
        }
        acc = 1;
        for i in 0..length {
            ret[length - 1 - i] *= acc;
            acc *= nums[length - 1 - i];
        }
        return ret;
    }
}
// impl Solution {
//     #[allow(dead_code)]
//     pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//         let (zero_count, product) = nums.iter().fold((0, 1), |(mut zero, mut prod), &x| {
//             if x == 0 {
//                 zero += 1;
//             } else {
//                 prod *= x;
//             }
//             (zero, prod)
//         });
//         let mut nums = nums;
//         match zero_count {
//             0 => {
//                 for n in nums.iter_mut() {
//                     *n = product / *n;
//                 }
//                 nums
//             }
//             1 => {
//                 for n in nums.iter_mut() {
//                     *n = if *n == 0 { product } else { 0 }
//                 }
//                 nums
//             }
//             _ => {
//                 vec![0; nums.len()]
//             }
//         }
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(vec![24, 12, 8, 6], Solution::product_except_self(nums));
    }
    #[test]
    fn test_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(vec![0, 0, 9, 0, 0], Solution::product_except_self(nums));
    }
}
