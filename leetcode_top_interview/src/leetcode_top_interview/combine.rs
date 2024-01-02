struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let total_size = Self::c_n_m(n, k) as usize;
        let mut ret = Vec::with_capacity(total_size);
        let mut tmp = Vec::with_capacity(k as usize);
        Self::push(1, n, k, &mut ret, &mut tmp);
        ret
    }
    fn c_n_m(n: i32, m: i32) -> i32 {
        if n == m {
            1
        } else {
            (m + 1..=n).product::<i32>() / (1..=(n - m)).product::<i32>()
        }
    }
    fn push(start: i32, end: i32, k: i32, ret: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>) {
        if k == 0 {
            ret.push(tmp.clone());
            return;
        }
        for first in start..=end - k + 1 {
            tmp.push(first);
            Self::push(first + 1, end, k - 1, ret, tmp);
            tmp.pop();
        }
    }
}
