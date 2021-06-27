// Other's solution
fn one_layer(n: usize, layer: usize) -> impl Iterator<Item = (usize, usize)> {
    let left_right = (layer..n - layer).map(move |i| (layer, i));
    let up_down = (layer + 1..n - layer).map(move |i| (i, n - 1 - layer));
    let right_left = (layer..n - 1 - layer).rev().map(move |i| (n - 1 - layer, i));
    let down_up = (layer + 1..n - 1 - layer).rev().map(move |i| (i, layer));

    left_right.chain(up_down).chain(right_left).chain(down_up)
}

pub fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    // n is the matrix size
    let n = matrix.get(0).map(|xs| xs.len()).unwrap_or(0);

    // circle_num is the number that snail number
    // 1 2 3     1 -> 2 -> 3 -> 6 -> 9 -> 8 -> 7 -> 4, is a circle.
    // 4 5 6
    // 7 8 9
    let circle_num = (n as f64 / 2.0).ceil() as usize;
    (0..circle_num)
        .flat_map(|x| one_layer(n, x))
        .map(|(x, y)| matrix[x][y])
        .collect()
}




// my solution
//
// enum Direction {
//     Left,
//     Top,
//     Right,
//     Down,
// }
// fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
//     if matrix.is_empty() {
//         return vec![];
//     }
//     let mut step = matrix.len() - 1;
//     let mut x = 0usize;
//     let mut y = step;
//     let mut ans = matrix[0].clone();
//     let mut dir = Direction::Down;
//     for _ in (1..=step).rev() {
//         for _ in 0..2 {
//             for _ in 0..step {
//                 match dir {
//                     Direction::Left => {
//                         y -= 1;
//                     }
//                     Direction::Top => {
//                         x -= 1;
//                     }
//                     Direction::Right => {
//                         y += 1;
//                     }
//                     Direction::Down => {
//                         x += 1;
//                     }
//                 }
//                 ans.push(matrix[x][y]);
//             }
//             match dir {
//                 Direction::Left => {
//                     dir = Direction::Top;
//                 }
//                 Direction::Top => {
//                     dir = Direction::Right;
//                 }
//                 Direction::Right => {
//                     dir = Direction::Down;
//                 }
//                 Direction::Down => {
//                     dir = Direction::Left;
//                 }
//             }
//         }
//         step -= 1;
//     }

//     ans
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
