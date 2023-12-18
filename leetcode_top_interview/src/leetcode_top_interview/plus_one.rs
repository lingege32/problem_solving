struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut digits = digits;
        for val in digits.iter_mut().rev() {
            *val += 1;
            if *val == 10 {
                *val = 0;
            } else {
                carry = 0;
                break;
            }
        }
        if carry == 1 {
            digits.push(1);
            digits.rotate_right(1);
        }
        digits
    }
}
