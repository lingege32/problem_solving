struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn add_binary2(a: String, b: String) -> String {
        let string_one = i128::from_str_radix(&a, 2).unwrap_or(0);
        let string_two = i128::from_str_radix(&b, 2).unwrap_or(0);
        format!("{:b}", string_one + string_two)
    }
}
impl Solution {


    #[allow(dead_code)]
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.into_bytes();
        let b = b.into_bytes();
        let (max, min) = if a.len() < b.len() { (b, a) } else { (a, b) };
        let min = Self::fill_vec(min, max.len());
        let mut ans = vec![0; max.len() + 1];
        let mut carry = 0;
        for idx in (1..=max.len()).rev() {
            let i = idx - 1;
            ans[idx] = Self::add(max[i], min[i], &mut carry);
        }

        if carry == 0 {
            std::str::from_utf8(&ans[1..]).unwrap().to_owned()
        } else {
            ans[0] = '1' as u8;
            std::str::from_utf8(&ans).unwrap().to_owned()
        }
    }
    fn add(a: u8, b: u8, carry: &mut u8) -> u8 {
        let ans = a - '0' as u8 + b - '0' as u8 + *carry;
        *carry = if ans > 1 { 1 } else { 0 };
        (ans & 1) + '0' as u8
    }
    fn fill_vec(mut min: Vec<u8>, len: usize) -> Vec<u8> {
        let min_len = min.len();
        min.resize(len, '0' as u8);
        min.rotate_left(min_len);
        min
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let a = "11".to_owned();
        let b = "1".to_owned();
        let ans = "100".to_owned();
        assert_eq!(ans, Solution::add_binary(a, b));
    }
}
