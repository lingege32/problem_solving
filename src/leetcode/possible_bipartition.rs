// other's solution achieve 100%

// impl Solution {
//     pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
//         let n = n as usize;
//         let mut representatives = vec![0; n + 1];
//         let mut uf = UnionFind::new(n + 1);
//         for dislike in &dislikes {
//             let (x, y) = (dislike[0] as usize, dislike[1] as usize);
//             if representatives[x] == 0 {
//                 representatives[x] = y;
//             } else {
//                 uf.union(representatives[x], y);
//             }
//             if representatives[y] == 0 {
//                 representatives[y] = x;
//             } else {
//                 uf.union(representatives[y], x);
//             }
//         }
//         for dislike in &dislikes {
//             let (x, y) = (dislike[0] as usize, dislike[1] as usize);
//             if uf.find(x) == uf.find(y) {
//                 return false;
//             }
//         }
//         return true;
//     }
// }

// struct UnionFind {
//     parents: Vec<usize>,
//     sizes: Vec<usize>,
// }

// impl UnionFind {
//     pub fn new(size: usize) -> Self {
//         Self {
//             parents: (0..size).collect(),
//             sizes: vec![1; size],
//         }
//     }

//     pub fn find(&mut self, x: usize) -> usize {
//         let mut root = x;
//         let Self { parents, .. } = self;
//         while parents[root] != root {
//             // Path splitting
//             let parent = parents[root];
//             parents[root] = parents[parent];
//             root = parent;
//         }
//         root
//     }

//     pub fn union(&mut self, x: usize, y: usize) -> bool {
//         let mut root1 = self.find(x);
//         let mut root2 = self.find(y);
//         if root1 == root2 {
//             return false;
//         }

//         // Union by size
//         let Self { parents, sizes } = self;
//         if sizes[root1] > sizes[root2] {
//             std::mem::swap(&mut root1, &mut root2);
//         }
//         parents[root1] = root2;
//         sizes[root2] += sizes[root1];
//         true
//     }
// }





use std::collections::HashSet;
struct Solution {}
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![(0u8, HashSet::new()); n as usize + 1];
        for edge in dislikes {
            graph[edge[0] as usize].1.insert(edge[1] as usize);
            graph[edge[1] as usize].1.insert(edge[0] as usize);
        }

        while let Some(root) = graph[1..].iter().enumerate().find(|x| x.1.0==0) {
            let root = root.0+1;
            
            let mut side = 1;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let vertex = queue.pop_front().unwrap();
                    if graph[vertex].0 == 0 {
                        graph[vertex].0 = side;
                        for adj in graph[vertex].1.iter() {
                            queue.push_back(*adj);
                        }
                    } else if graph[vertex].0 != side {
                        return false;
                    }
                }
                side = if side == 1 { 2 } else { 1 };
            }
    
        }
        
        true
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let a = [[1, 2], [1, 3], [2, 4]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(true, Solution::possible_bipartition(4, a));
    }

    #[test]
    fn test_2() {
        let a = [[1, 2], [1, 3], [2, 3]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(false, Solution::possible_bipartition(3, a));
    }

    #[test]
    fn test_3() {
        let a = [[1, 2], [2, 3], [3, 4], [4, 5], [1, 5]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(false, Solution::possible_bipartition(5, a));
    }
    #[test]
    fn test_4() {
        let a = [[1,2],[3,4],[4,5],[3,5]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(false, Solution::possible_bipartition(5, a));
    }
}
