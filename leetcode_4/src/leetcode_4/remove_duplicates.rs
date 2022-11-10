struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(s: String) -> String {
        let mut v = s.into_bytes();
        let mut idx = 0;
        for iter_idx in 0..v.len() {
            let &u = unsafe { v.get_unchecked(iter_idx) };
            if idx == 0 {
                unsafe {
                    *v.get_unchecked_mut(idx) = u;
                }
                idx += 1;
            } else {
                let &pre_u = unsafe { v.get_unchecked(idx - 1) };
                if pre_u == u {
                    idx -= 1;
                } else {
                    unsafe {
                        *v.get_unchecked_mut(idx) = u;
                    }
                    idx += 1;
                }
            }
        }
        v.resize(idx, 0 /*unused value */);
        unsafe { String::from_utf8_unchecked(v) }
    }
}
