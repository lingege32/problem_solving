struct Solution {
    
}
impl Solution {
    #[allow(dead_code)]
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points.into_iter().map(|x| (x[0]*x[0] + x[1]*x[1],x)).collect::<Vec<_>>();
        points.sort_unstable_by_key(|x| x.0);
        points.into_iter().take(k as usize).map(|x| x.1).collect()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let  points = [[1,3],[-2,2]].iter().map(|x| x.to_vec()).collect();
        let k = 1;
        let ans = [[-2,2]].iter().map(|x | x.to_vec()).collect::<Vec<_>>();
        assert_eq!(ans, Solution::k_closest(points, k));
    }

    #[test]
    fn test_2() {
        let  points = [[3,3],[5,-1],[-2,4]].iter().map(|x| x.to_vec()).collect();
        let k = 2;
        let ans = [[3,3],[-2,4]].iter().map(|x | x.to_vec()).collect::<Vec<_>>();
        assert_eq!(ans, Solution::k_closest(points, k));
    }
}