struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    #[allow(dead_code)]
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut bh = BinaryHeap::new();
        let mut v = Vec::with_capacity(profits.len());
        for (cap, pro) in capital.into_iter().zip(profits.into_iter()) {
            if pro <= 0 {
                continue;
            }
            if cap <= w {
                bh.push(pro);
            } else {
                v.push((std::cmp::Reverse(cap), pro));
            }
        }
        let mut candidate_heap = BinaryHeap::from(v);

        for _ in 0..k {
            while let Some(back) = candidate_heap.pop() {
                if back.0 .0 <= w {
                    bh.push(back.1);
                } else {
                    candidate_heap.push(back);
                    break;
                }
            }
            match bh.pop() {
                Some(pro) => {
                    w += pro;
                }
                None => {
                    break;
                }
            }
        }
        w
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let cap = vec![0, 1, 1];
        let pro = vec![1, 2, 3];
        let k = 2;
        let w = 0;
        let ans = 4;
        assert_eq!(ans, Solution::find_maximized_capital(k, w, pro, cap));
    }
    #[test]
    fn test_2() {
        let cap = vec![0, 1, 2];
        let pro = vec![1, 2, 3];
        let k = 3;
        let w = 0;
        let ans = 6;
        assert_eq!(ans, Solution::find_maximized_capital(k, w, pro, cap));
    }
}
