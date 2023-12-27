struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn hamming_weight(mut n: u32) -> i32 {
        // Method 1
        let mut ans = 0;
        while n != 0 {
            ans += 1;
            n = n & (n - 1);
        }
        ans
        // Method 2
        // n.count_ones() as i32

        // Method 3
        // (1..=32)
        //     .map(|x| if (n & (1 << x)) == 0 { 0 } else { 1 })
        //     .sum::<i32>()
    }
}
