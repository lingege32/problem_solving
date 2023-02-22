struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn ship_within_days_better(weights: Vec<i32>, days: i32) -> i32 {
        
        fn is_enough(capacity: i32, days: i32, weights: &Vec<i32>) -> bool{
            let mut total: i32 = 0;
            let mut count_days: i32 = 1; // starting day
            // for i in 0..weights.len(){
            for weight in weights.iter(){
                total += *weight;
                if total > capacity{
                    total = *weight;
                    count_days += 1;
                    if count_days > days{
                        return false;
                    }
                }
            }
            return true;
        }
        let mut start_capacity: i32 = *weights.iter().max().unwrap();
        let mut end_capacity: i32 = weights.iter().sum::<i32>();
        for weight in weights.iter(){
            end_capacity += *weight;
        }

        while start_capacity < end_capacity{
            let mid_capacity: i32 = start_capacity + (end_capacity-start_capacity)/2;
            if is_enough(mid_capacity, days, &weights){
                end_capacity = mid_capacity;
                continue;
            }
            start_capacity = mid_capacity + 1;
        }

        return start_capacity;
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        //
        // inclusive_sum waste many time...
        // 
        let mut weights = weights;
        for idx in 1..weights.len() {
            weights[idx] += weights[idx - 1];
        }
        let (mut left, mut right) = (0, *weights.last().unwrap());
        while left < right {
            let cargo = left + (right - left) / 2;
            if Self::helper(cargo, &weights, days) {
                right = cargo;
            } else {
                left = cargo + 1;
            }
        }
        left
    }
    fn helper(cargo: i32, inclusive_sum: &[i32], day: i32) -> bool {
        let mut left_cargo = 0;
        let mut left = 0;
        for _d in 1..=day {
            let mut right = inclusive_sum.len();
            while left < right {
                let mid = (left + right) / 2;
                if inclusive_sum[mid] - left_cargo <= cargo {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left_cargo = if left == 0 {
                0
            } else {
                inclusive_sum[left - 1]
            };
        }
        left == inclusive_sum.len()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let weights = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let days = 5;
        let ans = 15;
        assert_eq!(ans, Solution::ship_within_days(weights, days));
    }
    #[test]
    fn test_2() {
        let weights = vec![3, 2, 2, 4, 1, 4];
        let days = 3;
        let ans = 6;
        assert_eq!(ans, Solution::ship_within_days(weights, days));
    }
    #[test]
    fn test_3() {
        let weights = vec![1, 2, 3, 1, 1];
        let days = 4;
        let ans = 3;
        assert_eq!(ans, Solution::ship_within_days(weights, days));
    }
}
