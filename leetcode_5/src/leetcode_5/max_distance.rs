struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn max_distance_better(mut g: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let n = g.len();
        let mut queue = VecDeque::new();
        let mut distance = -1;
        for i in 0..n {
            for j in 0..n {
                if g[i][j] == 1 {
                    queue.push_back((i, j));
                }
            }
        }
        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                let (i, j) = queue.pop_front().unwrap();
                [(i-1,j), (i+1,j), (i,j-1), (i,j+1)]
                .iter()
                .for_each(|&(p,q)| {
                    if p < n && q < n && g[p][q] == 0 {
                        g[p][q] = 1;
                        queue.push_back((p,q));
                    }
                });
                size -= 1;
            }
            distance += 1;
        }
        if distance == 0 { -1 } else { distance }
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut starts = vec![];
        for (idx1, v) in grid.iter().enumerate() {
            for (idx2, vv) in v.iter().enumerate() {
                if *vv == 1 {
                    starts.push((idx1, idx2));
                }
            }
        }
        let mut map = vec![vec![-1; grid.len()]; grid.len()];
        for (idx1, idx2) in starts {
            Self::bfs(idx1, idx2, &grid, &mut map);
        }
        let ans = map.into_iter().flatten().max().unwrap();
        if ans == 0 {
            -1
        } else {
            ans
        }
    }
    fn bfs(idx1: usize, idx2: usize, grid: &Vec<Vec<i32>>, map: &mut Vec<Vec<i32>>) {
        map[idx1][idx2] = 0;
        let len = grid.len();
        let mut stack = vec![(idx1, idx2)];
        let mut dis = 1;
        while !stack.is_empty() {
            let mut new_stack = vec![];
            for (idx1, idx2) in stack.into_iter() {
                for (n_idx1, n_idx2) in Self::neighbor(idx1, idx2, len) {
                    let v = &mut map[n_idx1][n_idx2];
                    if grid[n_idx1][n_idx2] != 1 && (*v == -1 || *v > dis){
                        *v = dis;
                        new_stack.push((n_idx1, n_idx2));
                    }
                }
            }
            stack = new_stack;
            dis+=1;
        }
    }
    fn neighbor(idx1: usize, idx2: usize, bound: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];
        if idx1 != 0 {
            v.push((idx1 - 1, idx2));
        }
        if idx2 != 0 {
            v.push((idx1, idx2 - 1));
        }
        if idx1 + 1 != bound {
            v.push((idx1 + 1, idx2));
        }
        if idx2 + 1 != bound {
            v.push((idx1, idx2 + 1));
        }
        v
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![vec![1,0,1], vec![0,0,0], vec![1,0,1]];
        let ans = 2;
        assert_eq!(ans, Solution::max_distance(grid));
    }
}
