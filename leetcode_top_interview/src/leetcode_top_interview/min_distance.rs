struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.chars().into_iter().collect::<Vec<_>>();
        let word2 = word2.chars().into_iter().collect::<Vec<_>>();
        let mut dp1 = (0..=word2.len()).collect::<Vec<_>>();
        let mut dp2 = vec![0; word2.len()+1];
        for x in 1..=word1.len() {
            dp2[0] = x;
            for y in 1..=word2.len() {
                let c1 = word1[x - 1];
                let c2 = word2[y - 1];
                dp2[y] = if c1 == c2 {
                    dp1[y - 1]
                } else {
                    1 + dp1[y - 1].min(dp1[y]).min(dp2[y - 1])
                }
            }
            std::mem::swap(&mut dp1, &mut dp2);
            dp2.fill(0);
        }
        *dp1.last().unwrap() as i32
    }
}
