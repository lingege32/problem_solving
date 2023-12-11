struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut gas = gas;
        for idx in 0..len {
            gas[idx] -= cost[idx];
        }
        if gas.iter().sum::<i32>() < 0 {
            return -1;
        }

        let mut start_point = if gas[0] < 0 { 1 } else { 0 };

        for idx in 1..len {
            gas[idx] += gas[idx - 1];
            if gas[idx] < 0 {
                start_point = idx + 1;
                gas[idx] = 0;
            }
        }
        start_point as i32
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(3, Solution::can_complete_circuit(gas, cost));
    }
}
