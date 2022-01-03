struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_complement(num: i32) -> i32 {
        let mut mask: i32 = 0x4000_0000;
        let mut start = false;
        let mut ans = 0;
        while mask != 0 {
            let m = mask & num;
            if !start {
                if m != 0 {
                    start = true;
                }
            } else {
                ans *= 2;
                ans += if m == 0 { 1 } else { 0 };
            }
            // println!("mask: {:#06x}, start: {}", mask, start);
            // println!("num: {:#06x}", num);
            // println!("ans: {:#06x}", ans);

            mask = mask >> 1;
        }
        ans
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_complement(5));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::find_complement(1));
    }

    #[test]
    fn test3() {
        assert_eq!(13393220, Solution::find_complement(20161211));
    }
}
