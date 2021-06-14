
fn last_digit(lst: &[u64]) -> u64 {
    let mut r = 1;
    for &i in lst.iter().rev() {
        if i == 0 {
            match r {
                0 => r=1,
                _ => r=0
            };
        } else {
            let k = if r < 4 { r} else { r % 4 + 4 };
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
