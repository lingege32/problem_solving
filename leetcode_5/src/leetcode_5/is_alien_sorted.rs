struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut words = words
            .into_iter()
            .map(|x| x.into_bytes())
            .collect::<Vec<_>>();
        let mut map = [0; 26];
        let order = order.into_bytes();
        for i in 0..26 {
            map[(order[i] - 'a' as u8) as usize] = i;
        }
        for word in words.iter_mut() {
            word.iter_mut()
                .for_each(|x| *x = map[(*x - 'a' as u8) as usize] as u8);
        }
        !words.windows(2).any(|x| x[0] > x[1])
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let words = ["hello", "leetcode"]
            .iter()
            .map(|&x| x.to_owned())
            .collect::<Vec<_>>();
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_owned();
        let ans = true;
        assert_eq!(ans, Solution::is_alien_sorted(words, order));
    }
    #[test]
    fn test_2() {
        let words = ["word", "world", "row"]
            .iter()
            .map(|&x| x.to_owned())
            .collect::<Vec<_>>();
        let order = "worldabcefghijkmnpqstuvxyz".to_owned();
        let ans = false;
        assert_eq!(ans, Solution::is_alien_sorted(words, order));
    }
    #[test]
    fn test_3() {
        let words = ["apple", "app"]
            .iter()
            .map(|&x| x.to_owned())
            .collect::<Vec<_>>();
        let order = "abcdefghijklmnopqrstuvwxyz".to_owned();
        let ans = false;
        assert_eq!(ans, Solution::is_alien_sorted(words, order));
    }
}
