struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_pushes(word: String) -> i32 {
        let mut array = [0; 26];
        word.chars()
            .for_each(|x| array[x as usize - 'a' as usize] += 1);
        array.sort_unstable_by_key(|x| std::cmp::Reverse(*x));
        let mut count = 1;
        let mut retain_key = 8;
        array
            .into_iter()
            .map(|x| {
                if retain_key == 0 {
                    count += 1;
                    retain_key = 8;
                }
                retain_key-=1;
                count * x
            })
            .sum::<i32>()
    }
}
