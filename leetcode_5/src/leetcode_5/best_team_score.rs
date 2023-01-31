struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let max_age = *ages.iter().max().unwrap();
        let mut dp: Vec<i32> = vec![0; 1 + max_age as usize];
        let mut zip_scores: Vec<_> = scores
            .iter()
            .zip(ages.iter())
            .map(|(x, y)| (*x, *y))
            .collect();
        zip_scores.sort_unstable();
        zip_scores.iter().for_each(|(score, age)| {
            dp[*age as usize] = score + dp[..1 + *age as usize].iter().max().unwrap();
        });
        *dp.iter().max().unwrap()
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let scores = vec![1, 3, 5, 10, 15];
        let ages = vec![1, 2, 3, 4, 5];
        let ans = 34;
        assert_eq!(Solution::best_team_score(scores, ages), ans);
    }
    #[test]
    fn test_2() {
        let scores = vec![4, 5, 6, 5];
        let ages = vec![2, 1, 2, 1];
        let ans = 16;
        assert_eq!(Solution::best_team_score(scores, ages), ans);
    }
    #[test]
    fn test_3() {
        let scores = vec![1, 2, 3, 5];
        let ages = vec![8, 9, 10, 1];
        let ans = 6;
        assert_eq!(Solution::best_team_score(scores, ages), ans);
    }
    #[test]
    fn test_4() {
        let scores = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let ages = vec![811, 364, 124, 873, 790, 656, 581, 446, 885, 134];
        let ans = 10;
        assert_eq!(Solution::best_team_score(scores, ages), ans);
    }
    #[test]
    fn test_5() {
        let scores = vec![
            285, 779, 238, 299, 163, 857, 125, 875, 436, 547, 172, 873, 224, 618, 186, 700, 300,
            966, 198, 542, 343, 320, 337, 144, 684, 361, 745, 961, 898, 697, 319, 919, 846, 212,
            12, 658, 83, 684, 742, 505, 168, 996, 944, 235, 619, 629, 469, 750, 865, 836, 233, 65,
            620, 535, 406, 655, 968, 927, 926, 112, 983, 300, 877, 426, 736, 965, 787, 318, 817,
            76, 162, 489, 869, 708, 410, 27, 109, 535, 563, 717, 376, 292, 448, 351, 574, 113, 946,
            263, 421, 905, 33, 627, 256,
        ];
        let ages = vec![
            61, 36, 2, 3, 36, 71, 64, 78, 92, 12, 38, 35, 76, 94, 12, 93, 79, 83, 38, 67, 56, 81,
            91, 79, 76, 58, 9, 47, 18, 11, 17, 66, 74, 58, 14, 14, 67, 7, 17, 10, 70, 90, 56, 89,
            42, 43, 80, 67, 97, 16, 57, 73, 57, 83, 75, 94, 83, 9, 38, 70, 92, 65, 60, 51, 97, 96,
            14, 9, 31, 40, 52, 30, 53, 84, 73, 10, 90, 85, 36, 72, 90, 67, 13, 64, 100, 71, 84, 21,
            77, 60, 100, 5, 100,
        ];
        let ans = 10720;
        assert_eq!(Solution::best_team_score(scores, ages), ans);
    }
}
