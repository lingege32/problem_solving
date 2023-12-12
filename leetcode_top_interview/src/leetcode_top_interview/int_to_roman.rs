struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn int_to_roman(num: i32) -> String {
        let thousand = num / 1000;
        let hundred = num % 1000 / 100;
        let ten = num % 100 / 10;
        let one = num % 10;

        let mut ret = format!("{:M^1$}", "", thousand as usize);

        ret.push_str(&Self::get_s('M', 'D', 'C', hundred));
        ret.push_str(&Self::get_s('C', 'L', 'X', ten));
        ret.push_str(&Self::get_s('X', 'V', 'I', one));
        ret
    }

    fn get_s(ten: char, five: char, one: char, value: i32) -> String {
        match value {
            9 => {
                format!("{}{}", one, ten)
            }
            4 => {
                format!("{}{}", one, five)
            }
            5..=8 => {
                let mut s = format!("{}", five);
                for _ in 5..value {
                    s.push(one);
                }
                s
            }
            _ => {
                let mut s = String::new();
                for _ in 0..value {
                    s.push(one);
                }
                s
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let num = 3;
        assert_eq!("III".to_string(), Solution::int_to_roman(num));
    }
}
