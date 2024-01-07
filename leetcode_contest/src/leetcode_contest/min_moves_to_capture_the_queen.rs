struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        let mut queue = vec![(a, b, c, d, e, f)];
        let mut step = 1;
        while !queue.is_empty() {
            let mut next_queue = vec![];
            for (r1, r2, b1, b2, q1, q2) in queue.into_iter() {
                if Self::move_rook(r1, r2, b1, b2, q1, q2, &mut next_queue)
                    || Self::move_bishop(r1, r2, b1, b2, q1, q2, &mut next_queue)
                {
                    return step;
                }
            }
            step += 1;
            queue = next_queue;
        }
        std::unreachable!()
    }
    fn move_rook(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        queue: &mut Vec<(i32, i32, i32, i32, i32, i32)>,
    ) -> bool {
        for x1 in (1..a).rev() {
            if x1 == e && b == f {
                return true;
            }
            if x1 == c && b == d {
                break;
            }
            queue.push((x1, b, c, d, e, f));
        }
        for x1 in a + 1..=8 {
            if x1 == e && b == f {
                return true;
            }
            if x1 == c && b == d {
                break;
            }
            queue.push((x1, b, c, d, e, f));
        }
        for y1 in (1..b).rev() {
            if a == e && y1 == f {
                return true;
            }
            if a == c && y1 == d {
                break;
            }
            queue.push((a, y1, c, d, e, f));
        }
        for y1 in b + 1..=8 {
            if a == e && y1 == f {
                return true;
            }
            if a == c && y1 == d {
                break;
            }
            queue.push((a, y1, c, d, e, f));
        }
        false
    }
    fn move_bishop(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        queue: &mut Vec<(i32, i32, i32, i32, i32, i32)>,
    ) -> bool {
        let (mut next_c, mut next_d) = (c - 1, d - 1);
        while next_c > 0 && next_d > 0 && (a != next_c || b != next_d) {
            if next_c == e && next_d == f {
                return true;
            }
            queue.push((a, b, next_c, next_d, e, f));
            next_c -= 1;
            next_d -= 1;
        }
        (next_c, next_d) = (c + 1, d + 1);
        while next_c < 9 && next_d < 9 && (a != next_c || b != next_d) {
            if next_c == e && next_d == f {
                return true;
            }
            queue.push((a, b, next_c, next_d, e, f));
            next_c += 1;
            next_d += 1;
        }
        (next_c, next_d) = (c + 1, d - 1);
        while next_c > 0 && next_d > 0 && (a != next_c || b != next_d) {
            if next_c == e && next_d == f {
                return true;
            }
            queue.push((a, b, next_c, next_d, e, f));
            next_c += 1;
            next_d -= 1;
        }
        (next_c, next_d) = (c - 1, d + 1);
        while next_c < 9 && next_d < 9 && (a != next_c || b != next_d) {
            if next_c == e && next_d == f {
                return true;
            }
            queue.push((a, b, next_c, next_d, e, f));
            next_c -= 1;
            next_d += 1;
        }
        false
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::min_moves_to_capture_the_queen(4, 3, 3, 4, 5, 2)
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::min_moves_to_capture_the_queen(7, 3, 6, 4, 6, 7)
        );
    }
}
