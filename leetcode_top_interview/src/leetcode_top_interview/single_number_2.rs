struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // status from 00 -> 01 -> 11 -> 10 like a grey code.
        // b0
        // b1/b0   00 01 11 10
        //  num 0   0  1  1  x
        //      1   1  1  0  x
        // b0 = b0 & !num + !b1 & num
        // b1
        // b1/b0   00 01 11 10
        //  num 0   0  0  1  x
        //      1   0  1  0  x
        // b1 = b1 & !num + b0 & !b1 & num
        nums.into_iter()
            .fold((0, 0), |(b1, b0), num| {
                let new_b0 = (b0 & !num) | (!b1 & num);
                let new_b1 = (b1 & !num) | (b0 & !b1 & num);
                (new_b1, new_b0)
            })
            .1
    }
}
