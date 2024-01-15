struct Solution;
// Manacher’s algorithm
impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> String {
        let prime = Self::get_prime(s);
        // println!("prime: {:?}", prime);
        let mut radius = vec![0; prime.len()];
        let mut center = 0;
        for cur in 1..radius.len() {
            let rad = radius[center];
            let mut cur_radius = if cur > center + rad {
                0
            } else {
                let r = cur - center;
                radius[center-r].min(rad - r)
            };
            while cur - cur_radius > 0
                && cur + cur_radius + 1 < radius.len()
                && prime[cur - cur_radius - 1] == prime[cur + cur_radius + 1]
            {
                cur_radius += 1;
            }
            radius[cur] = cur_radius;
            if cur + cur_radius > center + rad {
                center = cur;
            }
        }
        // println!("radius: {:?}", radius);
        let (cen, rad) = radius.into_iter().enumerate().max_by_key(|x| x.1).unwrap();
        prime[cen - rad..=cen + rad]
            .iter()
            .filter(|c| **c != '#')
            .collect()
    }

    fn get_prime(s: String) -> Vec<char> {
        let mut ret = vec!['#'; s.len() * 2 + 1];
        for (idx, c) in s.chars().enumerate() {
            ret[idx * 2 + 1] = c;
        }
        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        println!("{}", Solution::longest_palindrome("babad".to_string()));
    }
    #[test]
    fn test_2() {
        println!("{}", Solution::longest_palindrome("a".to_string()));
    }
    #[test]
    fn test_3() {
        println!("{}", Solution::longest_palindrome("aaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()));
        println!("aaaaaaaaaaaaaaaaaaaaaaaaaa");
    }
    #[test]
    fn test_4() {
        println!("{}", Solution::longest_palindrome("aacabdkacaa".to_string()));
    }
}