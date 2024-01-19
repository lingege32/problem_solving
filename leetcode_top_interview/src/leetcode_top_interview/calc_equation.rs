struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let table = Self::make_table(equations, values);
        queries
            .into_iter()
            .map(|x| {
                let f = x[0].clone();
                let t = x[1].clone();
                if f == t {
                    if table.contains_key(&f) {
                        1.0
                    } else {
                        -1.0
                    }
                } else {
                    Self::find_solution(f, t, &table)
                }
            })
            .collect()
    }
    fn make_table(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
    ) -> HashMap<String, HashMap<String, f64>> {
        let mut table = HashMap::<String, HashMap<String, f64>>::new();
        for (equ, val) in equations.into_iter().zip(values.into_iter()) {
            let f = equ[0].to_string();
            let t = equ[1].to_string();
            let reciprocal = 1f64 / val;
            table
                .entry(f.clone())
                .or_default()
                .entry(t.clone())
                .or_insert(val);
            table
                .entry(t.clone())
                .or_default()
                .entry(f.clone())
                .or_insert(reciprocal);
        }
        table
    }
    fn find_solution(
        from: String,
        to: String,
        table: &HashMap<String, HashMap<String, f64>>,
    ) -> f64 {
        let mut queue = vec![(1.0, from)];
        let mut visited = HashSet::<String>::new();
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            for (v, name) in queue {
                if !visited.insert(name.clone()) {
                    continue;
                }
                if name == to {
                    return v;
                }
                if let Some(one_table) = table.get(&name) {
                    for (next_name, next_v) in one_table {
                        next_queue.push((v * next_v, next_name.clone()));
                    }
                }
            }
            queue = next_queue;
        }
        -1.0
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let equations = [["a", "b"], ["b", "c"]]
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let values = vec![2.0, 3.0];
        let queries = [["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let ans = vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000];
        assert_eq!(ans, Solution::calc_equation(equations, values, queries));
    }
}
