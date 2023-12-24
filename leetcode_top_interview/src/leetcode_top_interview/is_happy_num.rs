struct Solution;
use std::collections::HashSet;
impl Solution {
    #[allow(dead_code)]
    pub fn is_happy(mut n: i32) -> bool {
        let mut hs = HashSet::<i32>::new();
        while n != 1 && hs.insert(n) {
            n = Self::next_num(n);
        }

        n == 1
    }
    fn next_num(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            let m = n % 10;
            ans += m * m;
            n /= 10;
        }
        ans
    }
}
