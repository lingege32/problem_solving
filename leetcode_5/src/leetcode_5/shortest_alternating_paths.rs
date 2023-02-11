struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    NoColor,
    Red,
    Blue,
}
impl Solution {
    #[allow(dead_code)]
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let red_edges = Self::gen_map(red_edges);
        let blue_edges = Self::gen_map(blue_edges);
        let mut ret = vec![i32::MAX; n as usize];
        let mut visited = HashSet::<(Color, i32)>::new();

        let mut bfs = |cur: i32, color: Color, dis: i32, stack: &mut Vec<(i32, Color)>| {
            let insertion = visited.insert((color, cur));
            if insertion {
                let val = ret.get_mut(cur as usize).unwrap();
                *val = dis.min(*val);
                if color != Color::Red {
                    if let Some(all_to) = red_edges.get(&cur) {
                        for to in all_to {
                            stack.push((*to, Color::Red));
                        }
                    }
                }
                if color != Color::Blue {
                    if let Some(all_to) = blue_edges.get(&cur) {
                        for to in all_to {
                            stack.push((*to, Color::Blue));
                        }
                    }
                }
            }
        };
        let mut stack = vec![];
        bfs(0, Color::NoColor, 0, &mut stack);
        let mut dis = 1;
        while !stack.is_empty() {
            let mut next_stack = vec![];
            for (n, color) in stack {
                bfs(n, color, dis, &mut next_stack);
            }
            stack = next_stack;
            dis += 1;
        }
        ret.iter_mut().for_each(|x| {
            if *x == i32::MAX {
                *x = -1;
            }
        });
        ret
    }

    fn gen_map(edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut hm = HashMap::new();
        for pair in edges {
            let (from, to) = (pair[0], pair[1]);
            hm.entry(from).or_insert(vec![]).push(to);
        }
        hm
    }
}

struct OtherSolution;
use std::collections::VecDeque;

impl OtherSolution {
    #[allow(dead_code)]
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, bool)>> = vec![Vec::new(); n];
        red_edges
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize, true))
            .chain(
                blue_edges
                    .into_iter()
                    .map(|v| (v[0] as usize, v[1] as usize, false)),
            )
            .for_each(|(from, to, is_red)| graph[from].push((to, is_red)));

        let mut distance: Vec<(i32, i32)> = vec![(i32::MAX, i32::MAX); n];
        let mut queue: VecDeque<(usize, bool)> = VecDeque::new();
        queue.push_back((0, true));
        queue.push_back((0, false));
        distance[0] = (0, 0);

        while let Some((node, is_red)) = queue.pop_front() {
            let new_distance = if is_red {
                distance[node].0 + 1
            } else {
                distance[node].1 + 1
            };
            for &edge in graph[node].iter() {
                if edge.1 == is_red {
                    continue;
                }
                if is_red {
                    if distance[edge.0].1 > new_distance {
                        queue.push_back(edge);
                        distance[edge.0].1 = new_distance;
                    }
                } else {
                    if distance[edge.0].0 > new_distance {
                        queue.push_back(edge);
                        distance[edge.0].0 = new_distance;
                    }
                }
            }
        }

        distance
            .into_iter()
            .map(|(red, blue)| {
                let result = std::cmp::min(red, blue);
                if result == i32::MAX {
                    -1
                } else {
                    result
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let n = 5;
        let red_edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        let blue_edges = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        let ans = vec![0, 1, 2, 3, 7];
        assert_eq!(
            ans,
            Solution::shortest_alternating_paths(n, red_edges, blue_edges)
        );
    }
}
