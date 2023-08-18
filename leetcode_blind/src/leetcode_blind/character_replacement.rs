struct Solution;

use std::cmp::max;
const NUM_ALPHA: usize = 26;

impl Solution {
    #[allow(dead_code)]
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut counts: [usize; NUM_ALPHA] = [0; NUM_ALPHA];
        let s = s.chars().collect::<Vec<_>>();
        (0..s.len())
            .fold((0, 0, 0), |(res_len, mut maxf, mut l), r| {
                counts[(s[r] as u8 - b'A') as usize] += 1;
                maxf = max(maxf, counts[(s[r] as u8 - b'A') as usize]);

                // subarray size - maxf means we need to change charater at that num.
                // The max subarray size is at least maxf+k.
                // Therefore, we don't care that remove the most frequency charater in the left.
                // Just remove and wait for the character with more frequent.
                if (r - l + 1) - maxf > k as usize {
                    counts[(s[l] as u8 - b'A') as usize] -= 1;
                    l += 1;
                }
                (max(res_len, r - l + 1), maxf, l)
            })
            .0 as i32
    }
}
