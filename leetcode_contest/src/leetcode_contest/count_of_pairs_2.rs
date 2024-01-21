struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let (x, y) = if x < y { (x, y) } else { (y, x) };
        let mut dis_l_in_circle = vec![0; n as usize + 1];
        let mut dis_r_in_circle = vec![0; n as usize + 1];
        for i in x + 1..y {
            let tmp_l = (i - x).min(y - i + 1);
            let tmp_r = (y - i).min(i - x + 1);
            dis_l_in_circle[tmp_l as usize] += 1;
            dis_r_in_circle[tmp_r as usize] += 1;
        }
        println!("{:?}", dis_l_in_circle);
        println!("{:?}", dis_r_in_circle);
        let longest_path = n - 0.max(y - x - 1);
        let circle = y - x + 1;
        let mut suml = 0;
        let mut sumr = 0;
        let mut l1 = -(x - 2);
        let mut r1 = 0;
        let mut l2 = -(n - y - 1);
        let mut r2 = 0;
        let mut ans = vec![0; n as usize];
        for idx in 0..n as usize{
            let dis = (idx + 1) as i32;
            // case1 : for the path outside the circle
            ans[idx] += 0.max(longest_path - dis);
            // case2 : Consider the houses within the range
            if dis * 2 < circle {
                ans[idx] += circle;
            } else if dis * 2 == circle {
                ans[idx] += circle / 2;
            }

            // case 3 : one end point is in (x,y) and the other is not.
            ans[idx] += suml + sumr;
            println!("dis: {dis}, suml: {suml}, l1: {l1}, r1: {r1}, sumr: {sumr}, l2: {l2}, r2: {r2}");
            // update suml and sumr by removing outdated distance
            if l1 >= 0 {
                suml -= dis_l_in_circle[l1 as usize];
            }
            if l2 >= 0 {
                sumr -= dis_r_in_circle[l2 as usize];
            }

            l1 += 1;
            r1 += 1;
            l2 += 1;
            r2 += 1;

            suml += dis_l_in_circle[r1 as usize];
            sumr += dis_r_in_circle[r2 as usize];
        }
        if x != y {
            ans[0] -= 1;
        }
        ans.iter_mut().for_each(|x| {
            (*x) *= 2;
        });
        ans
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let x = 1;
        let y = 4;
        println!("{:?}", Solution::count_of_pairs(n, x, y));
    }
    #[test]
    fn test_1123() {
        let n = 10;
        let x = 4;
        let y = 8;
        println!("{:?}", Solution::count_of_pairs(n, x, y));
    }
    #[test]
    fn test_12() {
        let n = 4;
        let x = 1;
        let y = 1;
        println!("{:?}", Solution::count_of_pairs(n, x, y));
    }
    #[test]
    fn test_132() {
        let n = 5;
        let x = 2;
        let y = 4;
        println!("{:?}", Solution::count_of_pairs(n, x, y));
    }
}
