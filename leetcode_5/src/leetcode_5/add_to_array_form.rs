struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn add_to_array_form2(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut out = vec![];
        let mut carry = 0;

        let mut num1_iter = num.into_iter().map(|n| n as u32).rev();
        let k = format!("{k}");
        let mut num2_iter = k.chars().map(|c| c.to_digit(10).unwrap()).rev();

        let mut num1 = num1_iter.next();
        let mut num2 = num2_iter.next();

        while num1.is_some() || num2.is_some() || carry > 0 {
            let n1 = num1.unwrap_or_default();
            let n2 = num2.unwrap_or_default();
            let sum = n1 + n2 + carry;

            out.push((sum % 10) as i32);

            num1 = num1_iter.next();
            num2 = num2_iter.next();
            carry = sum / 10;
        }

        out.reverse();
        out
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn add_to_array_form(mut num: Vec<i32>, k: i32) -> Vec<i32> {
        num.reverse();
        let k = Self::to_vec(k);
        let (max, min) = if num.len() < k.len() {
            (k, num)
        } else {
            (num, k)
        };
        Self::add(max, min)
    }
    fn to_vec(mut k: i32) -> Vec<i32> {
        let mut ans = vec![];
        while k != 0 {
            ans.push(k % 10);
            k /= 10;
        }
        ans
    }
    fn add(mut max: Vec<i32>, min: Vec<i32>) -> Vec<i32> {
        println!("max: {:?}, min: {:?}", max, min);
        let mut carry = 0;
        let get = |a, b: &mut i32, c: &mut i32| {
            if a > 9 {
                *b = a - 10;
                *c = 1;
            } else {
                *b = a;
                *c = 0;
            }
        };
        for i in 0..min.len() {
            let a = max[i] + min[i] + carry;
            get(a, &mut max[i], &mut carry);
        }
        for i in min.len()..max.len() {
            if carry == 0 {
                break;
            }
            let a = max[i] + 1;
            get(a, &mut max[i], &mut carry);
        }
        if carry == 1 {
            max.resize(max.len() + 1, 1);
        }
        max.reverse();
        max
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let num = vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
        let k = 1;
        let ans = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(ans, Solution::add_to_array_form(num, k));
    }
    #[test]
    fn test_2() {
        let num = vec![2, 1, 5];
        let k = 806;
        let ans = vec![1, 0, 2, 1];
        assert_eq!(ans, Solution::add_to_array_form(num, k));
    }
}
