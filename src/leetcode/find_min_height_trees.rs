// Other's Solution
// use std::collections::HashMap;
// impl Solution {
//     pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
//         if edges.len() == 0 {
//             return vec![0];
//         }
//         let mut degree = vec![0; n as usize];
//         let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
//         for edge in edges {
//             if let Some(a) = adj.get_mut(&edge[0]) {
//                 (*a).push(edge[1]);
//             } else {
//                 adj.insert(edge[0], vec![edge[1]]);
//             }
//             if let Some(a) = adj.get_mut(&edge[1]) {
//                 (*a).push(edge[0]);
//             } else {
//                 adj.insert(edge[1], vec![edge[0]]);
//             }
//             degree[edge[0] as usize] += 1;
//             degree[edge[1] as usize] += 1;
//         }

//         let mut queue = vec![];
//         for i in 0..n {
//             if degree[i as usize] == 1 {
//                 queue.push(i);
//             }
//         }

//         let mut v = n;
//         while v > 2 {
//             v -= queue.len() as i32;
//             for _i in 0..queue.len() {
//                 // let t = *queue.first().unwrap() as i32;
//                 let t = queue.remove(0) as i32;
//                 for &j in adj.get(&t).unwrap() {
//                     degree[j as usize] -= 1;
//                     if degree[j as usize] == 1 {
//                         queue.push(j);
//                     }
//                 }
//             }
//         }
//         return queue;
//         // let mut result = vec![];
//         // println!("debug queue {:?}", queue);
//         // while !queue.is_empty() {
//         //     result.push(queue.remove(0) as i32);
//         // }
//         // println!("debug {:?}", result);
//         // return result;
//     }
// }
















// my Solution

use std::{collections::HashSet, hash::Hasher};
struct Solution();
struct Graph {
    node_size: usize,
    m_g: Vec<HashSet<usize>>,
}
impl Graph {
    pub fn with_node_size(n: usize) -> Self {
        Graph {
            node_size: n,
            m_g: vec![HashSet::new(); n],
        }
    }
    pub fn add_edges(&mut self, edges: &Vec<Vec<i32>>) {
        edges.iter().for_each(|x| {
            self.m_g[x[0] as usize].insert(x[1] as usize);
            self.m_g[x[1] as usize].insert(x[0] as usize);
        });
    }
    pub fn tree_height_by_root(&self, root: usize) -> usize {
        // use bfs, complexity is O(V)
        let mut visited = vec![false; self.node_size];
        let mut current_layer = vec![root];
        let mut next_layer = Vec::<usize>::new();
        let mut height = 0;
        while !current_layer.is_empty() {
            height += 1;
            for &node in current_layer.iter() {
                visited[node] = true;
                for &next_node in self.m_g[node].iter() {
                    if visited[next_node] {
                        continue;
                    }
                    next_layer.push(next_node);
                }
            }
            std::mem::swap(&mut next_layer, &mut current_layer);
            next_layer.clear();
        }
        height
    }

    pub fn find_min_height_tree(mut self) -> Vec<i32> {
        // complexity O(V+E)
        let mut prev_leaf_vec = Vec::new();
        let mut leaf_vec = Vec::new();
        let edges = &mut self.m_g;
        for (ver, edge) in edges.iter().enumerate() {
            if edge.len() == 1 {
                leaf_vec.push(ver);
            }
        }

        while !leaf_vec.is_empty() {
            std::mem::swap(&mut leaf_vec, &mut prev_leaf_vec);
            leaf_vec.clear();
            for &vertex in prev_leaf_vec.iter() {
                if let Some(&adjacent) = edges[vertex].iter().next() {
                    edges[vertex].remove(&adjacent);
                    edges[adjacent].remove(&vertex);

                    if edges[adjacent].len() == 1 {
                        leaf_vec.push(adjacent);
                    }
                }
            }
        }

        prev_leaf_vec.into_iter().map(|x| x as i32).collect()
    }
}
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 0 {
            return vec![];
        } else if n == 1 {
            return vec![0];
        } else if n == 2 {
            return vec![0, 1];
        }
        let mut graph = Graph::with_node_size(n as usize);
        graph.add_edges(&edges);
        graph.find_min_height_tree()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let a = [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        assert_eq!([3, 4].to_vec(), Solution::find_min_height_trees(6, a));
    }
}
