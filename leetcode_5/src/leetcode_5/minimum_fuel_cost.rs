struct Solution;

impl Solution {
    #[allow(dead_code)]
    // other's good readability
    pub fn better_solution(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut adj = vec![vec![]; n];
        for r in roads {
            adj[r[0] as usize].push(r[1] as usize);
            adj[r[1] as usize].push(r[0] as usize);
        }

        let mut visited = vec![false; n];
        fn explore(
            adj: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            u: usize,
            seats: i64,
        ) -> (i64, i64) {
            visited[u] = true;
            let (mut riders, mut liters) = (0, 0);
            for &v in &adj[u] {
                if !visited[v] {
                    let (r2, l2) = explore(adj, visited, v, seats);
                    riders += r2;
                    liters += l2 + (r2 + seats - 1) / seats;
                }
            }
            (riders + 1, liters)
        }
        explore(&adj, &mut visited, 0, seats as i64).1
    }





    #[allow(dead_code)]
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        if roads.is_empty() {
            return 0;
        }
        let n = roads.len() + 1;
        let mut visited = vec![false; n as usize];
        let hm = {
            let mut hm = vec![vec![]; n];
            for edge in roads {
                let (n1, n2) = (edge[0], edge[1]);
                hm[n1 as usize].push(n2);
                hm[n2 as usize].push(n1);
            }
            hm
        };
        let mut ret = 0;
        fn dfs(
            country: i32,
            visited: &mut [bool],
            hm: &[Vec<i32>],
            ret: &mut i64,
            seats: i32,
        ) -> i32 {
            visited[country as usize] = true;
            let next = hm[country as usize]
                .iter()
                .filter(|&&x| !visited[x as usize])
                .collect::<Vec<_>>();
            if next.is_empty() {
                1
            } else {
                next.iter()
                    .map(|&&n| {
                        let total = dfs(n, visited, hm, ret, seats);
                        *ret += ((total + seats - 1) / seats) as i64;

                        total
                    })
                    .sum::<i32>()
                    + 1
            }
        }

        dfs(0, &mut visited, &hm, &mut ret, seats);

        ret
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let roads = vec![
            vec![3, 1],
            vec![3, 2],
            vec![1, 0],
            vec![0, 4],
            vec![0, 5],
            vec![4, 6],
        ];
        let seats = 2;
        let ans = 7;
        assert_eq!(ans, Solution::minimum_fuel_cost(roads, seats));
    }
    #[test]
    fn test_2() {
        let roads = vec![vec![0, 1], vec![0, 2], vec![0, 3]];
        let seats = 5;
        let ans = 3;
        assert_eq!(ans, Solution::minimum_fuel_cost(roads, seats));
    }
}
