struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        fn multiply_u8(num1: &[u8], num2: &[u8]) -> String {
            let mut ans: Vec<u8> = vec![0; num1.len() + num2.len()];
            fn mul(multiplicand: &[u8], one: u8, how_many_zero: usize) -> Vec<u8> {
                let mut res = Vec::with_capacity(multiplicand.len() + 1 + how_many_zero);
                (0..how_many_zero).for_each(|_| res.push(0));
                let mut carry = 0;
                for &val in multiplicand.iter() {
                    let sum = carry + val * one;
                    carry = sum / 10;
                    res.push(sum % 10);
                }
                if carry > 0 {
                    res.push(carry);
                }
                res
            }
            fn add_to_ans(ans: &mut Vec<u8>, added: Vec<u8>) {
                // added always is bigger than ans
                let mut carry = 0;
                let added_len = added.len();
                for idx in 0..added_len {
                    let sum = carry + ans[idx] + added[idx];
                    carry = sum / 10;
                    ans[idx] = sum % 10;
                }
                if carry > 0 {
                    ans[added_len] += carry;
                }
            }

            for (idx, val) in num2.iter().enumerate() {
                add_to_ans(&mut ans, mul(&num1, *val, idx));
            }
            let (last_idx, _) = ans
                .iter()
                .rev()
                .enumerate()
                .find(|&(_, y)| *y != 0)
                .unwrap();
            ans.truncate(ans.len() - last_idx);

            ans.reverse();
            ans.iter_mut().for_each(|x| *x += '0' as u8);
            unsafe { String::from_utf8_unchecked(ans) }
        }
        let mut new_num1 = (num1.into_bytes())
            .into_iter()
            .map(|x| x - '0' as u8)
            .collect::<Vec<u8>>();
        let mut new_num2 = (num2.into_bytes())
            .into_iter()
            .map(|x| x - '0' as u8)
            .collect::<Vec<u8>>();
        new_num1.reverse();
        new_num2.reverse();
        if new_num1.len() < new_num2.len() {
            std::mem::swap(&mut new_num1, &mut new_num2);
        }
        multiply_u8(&new_num1, &new_num2)
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        assert_eq!(
            Solution::multiply("2".to_owned(), "3".to_owned()),
            "6".to_owned()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::multiply("123".to_owned(), "456".to_owned()),
            "56088".to_owned()
        );
    }
    #[test]
    fn test_23() {
        assert_eq!(
            Solution::multiply("9".to_owned(), "9".to_owned()),
            "81".to_owned()
        );
    }
}
