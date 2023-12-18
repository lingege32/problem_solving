struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut ans = 0;
        for shift in (0..32).rev() {
            ans |= (x & 0x1) << shift;
            x >>= 1;
        }
        ans
    }
}
