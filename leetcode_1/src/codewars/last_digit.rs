
// solution : https://learnku.com/articles/30221
fn last_digit(lst: &[u64]) -> u64 {
    let mut r = 1;
    for &i in lst.iter().rev() {
        if i == 0 {
            match r {
                0 => r=1,
                _ => r=0
            };
        } else {
            // 我們需要兩個保證來保證個位數必須保持一致
            // 1. 為了使 m^n 的個位數保持一致，會發現除了n!=0的狀況下為4個一循環
            // 2. 為了讓 i^(m^n) 個位數也保持一致，(m^n)%4 在 (m^n) 大於2的時候是2個一循環的
            // 所以在 r > 2 (2.) 的時候 可以把數字直接化簡4個一循環(1.)來優化 pow() 的performance
            let k = if r < 2 { r} else { r % 4 + 4 };

            // i is too large, will cause the u32 overflow
            // (i%100) 可以保持個位數一致
            r = (i%100).pow(k as u32);
        }
    }
    r%10
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        let tests = vec![
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6),
            (vec![499942, 999998, 999999], 6),
        ];

        for test in tests {
            assert_eq!(last_digit(&test.0), test.1);
        }
    }
}
