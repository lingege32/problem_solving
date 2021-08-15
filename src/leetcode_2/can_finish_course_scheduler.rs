struct Solution {}
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        fn dfs(root: usize, graph: &Vec<Vec<usize>>, visited: &mut [u8]) -> bool {
            match visited[root] {
                1 => false,
                2 => true,
                _ => {
                    visited[root] = 1;
                    for adjacent in &graph[root] {
                        if !dfs(*adjacent, graph, visited) {
                            return false;
                        }
                    }
                    visited[root] = 2;
                    true
                }
            }
        }
        let a =
            prerequisites
                .into_iter()
                .fold(vec![vec![]; num_courses as usize], |mut acc, elem| {
                    acc[elem[0] as usize].push(elem[1] as usize);
                    acc
                });
        let mut visited = vec![0; num_courses as usize];
        for root in 0..num_courses as usize {
            if !dfs(root, &a, &mut visited) {
                return false;
            }
        }
        true
    }
    /* my first solution, performance is very bad because we dfs at the same point with multiple times */
    // pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    //     fn dfs(pre: usize, next: usize, graph: &Vec<Vec<usize>>) -> bool {
    //         let mut stack = vec![];
    //         stack.push(pre);
    //         while let Some(n) = stack.pop() {
    //             if next == n {
    //                 return true;
    //             }
    //             for i in &graph[n] {
    //                 stack.push(*i);
    //             }
    //         }
    //         false
    //     }
    //     let mut graph = vec![vec![]; num_courses as usize];
    //     for constraint in prerequisites {
    //         let (pre, next) = (constraint[1] as usize, constraint[0] as usize);
    //         if pre==next {
    //             return false;
    //         }
    //         if graph[next].is_empty() || !dfs(next, pre, &graph) {
    //             graph[pre].push(next);
    //         } else {
    //             return false;
    //         }
    //     }
    //     true
    // }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let prerequisites = [[1, 0]].iter().map(|x| x.to_vec()).collect();
        assert!(Solution::can_finish(2, prerequisites));
    }
    #[test]
    fn test_2() {
        let prerequisites = [[1, 0], [0, 1]].iter().map(|x| x.to_vec()).collect();
        assert!(!Solution::can_finish(2, prerequisites));
    }
    #[test]
    fn test_3() {
        let prerequisites = [
            [0, 10],
            [3, 18],
            [5, 5],
            [6, 11],
            [11, 14],
            [13, 1],
            [15, 1],
            [17, 4],
        ]
        .iter()
        .map(|x| x.to_vec())
        .collect();
        assert!(!Solution::can_finish(20, prerequisites));
    }
}
