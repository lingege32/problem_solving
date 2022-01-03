// Other's Solution achieve 100%

struct Solution();
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        is_bipartite(
            &graph
                .into_iter()
                // These really ought to have been unsigned all along, if the graph is represented as a Vec
                .map(|edges| edges.into_iter().map(|edge| edge as usize).collect())
                .collect::<Vec<_>>(),
        )
    }
}

pub fn is_bipartite(graph: &[Vec<usize>]) -> bool {
    #[derive(Clone, Copy, PartialEq)]
    enum Set {
        A,
        B,
    }

    impl Set {
        fn other(&self) -> Self {
            use Set::*;

            match self {
                A => B,
                B => A,
            }
        }
    }

    fn visit(
        node: usize,
        set: Set,
        graph: &[Vec<usize>],
        visited: &mut Vec<Option<Set>>,
    ) -> bool {
        match visited[node] {
            None => {
                visited[node] = Some(set);
                graph[node]
                    .iter()
                    .map(|&node| visit(node, set.other(), graph, visited))
                    .all(|x| x)
            }
            Some(s) if s != set => false,
            _ => true,
        }
    }

    let mut visited = vec![None; graph.len()];

    (0..graph.len()).all(|node| {
        if visited[node].is_none() {
            visit(node, Set::A, graph, &mut visited)
        } else {
            true
        }
    })
}


#[cfg(test)]
mod test_super {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_1() {
        let graph = [vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]].to_vec();
        assert!(!Solution::is_bipartite(graph));
    }
    #[test]
    fn test_2() {
        let graph = [[1, 3], [0, 2], [1, 3], [0, 2]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert!(Solution::is_bipartite(graph));
    }
    #[test]
    fn test_3() {
        let graph = [
            vec![],
            vec![2, 4, 6],
            vec![1, 4, 8, 9],
            vec![7, 8],
            vec![1, 2, 8, 9],
            vec![6, 9],
            vec![1, 5, 7, 8, 9],
            vec![3, 6, 9],
            vec![2, 3, 4, 6, 9],
            vec![2, 4, 5, 6, 7, 8],
        ]
        .to_vec();
        assert!(!Solution::is_bipartite(graph));
    }
}
