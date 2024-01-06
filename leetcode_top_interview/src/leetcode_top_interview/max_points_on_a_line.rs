struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() < 3 {
            return points.len() as i32;
        }
        let mut dp =
            vec![std::collections::HashMap::<(i32, i32), (usize, i32)>::new(); points.len()];
        for j in 1..points.len() {
            let v2 = &points[j];
            let (x2, y2) = (v2[0], v2[1]);
            for (i, v1) in points[..j].iter().enumerate() {
                let (x1, y1) = (v1[0], v1[1]);
                let slope = Self::get_slope(y2 - y1, x2 - x1);
                let (first_node, count) = dp[i]
                    .get(&slope)
                    .map(|&(prev, count)| (prev, count + 1))
                    .unwrap_or((i, 2));
                dp[j].insert(slope, (first_node, count));
            }
        }
        dp.into_iter()
            .map(|x: std::collections::HashMap<(i32, i32), (usize, i32)>| {
                x.into_iter().map(|y| y.1 .1).max().unwrap_or(0)
            })
            .max()
            .unwrap()
    }

    fn get_slope(mut y: i32, mut x: i32) -> (i32, i32) {
        if x == 0 {
            return (1, 0);
        }
        if y == 0 {
            return (0, 1);
        }
        let mut sign = 1;
        if y < 0 {
            y = -y;
            sign *= -1;
        }
        if x < 0 {
            x = -x;
            sign *= -1;
        }
        let lcd = if x < y {
            Self::lcd(x, y)
        } else {
            Self::lcd(y, x)
        };
        return (sign * y / lcd, x / lcd);
    }
    fn lcd(x: i32, y: i32) -> i32 {
        if x == 0 {
            y
        } else {
            Self::lcd(y % x, x)
        }
    }
}

use std::collections::HashMap;
struct Solution2;
impl Solution2 {
    #[allow(dead_code)]
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<(i64, i64), i32> = HashMap::new();
        let mut maxim = 0;
        for i in 1..points.len() {
            for j in 0..i {
                if points[i][0] == points[j][0] {
                    *map.entry((i64::MAX, points[i][1] as i64)).or_default() += 1;
                    continue;
                }
                let k =
                    ((points[i][1] - points[j][1]) as f64) / ((points[i][0] - points[j][0]) as f64);
                let b = (points[i][1] as f64) - k * points[i][0] as f64;
                *map.entry((Self::amplify(k), Self::amplify(b))).or_default() += 1;
            }
            maxim = maxim.max(*map.values().max().unwrap());
            map.drain();
        }
        maxim + 1
    }
    fn amplify(val: f64) -> i64 {
        (val * 1_000_000_000_000.0).round() as i64
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let points = [[1, 1], [2, 2], [3, 3]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let ans = 3;
        assert_eq!(ans, Solution::max_points(points));
    }
}
