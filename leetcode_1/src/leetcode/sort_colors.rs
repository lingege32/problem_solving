struct Solution {
}
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut r, mut g, mut b) = (0,0,0);
        nums.iter().for_each(|x| {
            match x {
                0 => r+=1,
                1 => g+=1,
                2 => b+=1,
                _ => {}
            }
        });
        for item in nums.iter_mut().take(r) {
            *item = 0;
        }
        for item in nums.iter_mut().skip(r).take(r+g) {
            *item = 1;
        }
        for item in nums.iter_mut().skip(r+g).take(b) {
            *item = 2;
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = [2,0,2,1,1,0].to_vec();
        Solution::sort_colors(&mut nums);
        assert_eq!([0,0,1,1,2,2].to_vec(), nums);
    }
    #[test]
    fn test_2() {
        let mut nums = [2,0,1].to_vec();
        Solution::sort_colors(&mut nums);
        assert_eq!([0,1,2].to_vec(), nums);
    }
    #[test]
    fn test_3() {
        let mut nums = [0].to_vec();
        Solution::sort_colors(&mut nums);
        assert_eq!([0].to_vec(), nums);
    }
    #[test]
    fn test_4() {
        let mut nums = [1].to_vec();
        Solution::sort_colors(&mut nums);
        assert_eq!([1].to_vec(), nums);
    }
}
