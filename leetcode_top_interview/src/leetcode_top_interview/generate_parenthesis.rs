struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret = vec![];
        Self::inner(n, n, "".to_string(), &mut ret);
        ret
    }
    fn inner(left: i32, right: i32, s: String, ret: &mut Vec<String>) {
        if left > right {
            return;
        }
        if right == 0 && left == 0 {
            ret.push(s);
        } else {
            if left > 0 {
                Self::inner(left - 1, right, format!("{}(", s), ret);
            }
            if right > 0 {
                Self::inner(left, right - 1, format!("{})", s), ret);
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        println!("1. {:?}", Solution::generate_parenthesis(1));
        println!("2. {:?}", Solution::generate_parenthesis(2));
        println!("3. {:?}", Solution::generate_parenthesis(3));
        println!("4. {:?}", Solution::generate_parenthesis(4));
        println!("5. {:?}", Solution::generate_parenthesis(5));
        println!("6. {:?}", Solution::generate_parenthesis(6));
        println!("7. {:?}", Solution::generate_parenthesis(7));
        println!("8. {:?}", Solution::generate_parenthesis(8));
    }
}
