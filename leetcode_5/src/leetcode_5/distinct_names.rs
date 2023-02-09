struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        use std::collections::HashSet;
        use std::collections::HashMap;

        // init buckets
        let mut buckets = HashMap::new();
        for idea in ideas {
            let c = idea[..1].to_string();
            let w = idea[1..].to_string();
            let bucket = buckets.entry(c).or_insert(HashSet::new());
            bucket.insert(w);
        }

        let keys: Vec<&String> = buckets.keys().collect();
        let mut cnt = 0;

        for i in 0..keys.len() - 1 {
            for j in i + 1..keys.len() {
                let bucket1 = buckets.get(keys[i]).unwrap();
                let bucket2 = buckets.get(keys[j]).unwrap();
                /*
                a | b,c,f
                c | b,c,e,g
                lhs: f
                rhs: e, g
                 */
                let lhs: Vec<&String> = bucket1.difference(bucket2).collect();
                let rhs: Vec<&String> = bucket2.difference(bucket1).collect();
                cnt += lhs.len() * rhs.len() * 2;
            }
        }

        cnt as i64
    }
}
