struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let start = Self::trans(start_gene);
        let end = Self::trans(end_gene);
        let mut bank = bank.into_iter().map(|x| Self::trans(x)).collect::<Vec<_>>();
        bank.push(start);
        let mut visit = vec![false; bank.len()];
        let mut queue = vec![bank.len()-1];
        let mut level = 0;
        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            for idx in queue {
                if visit[idx]  {
                    continue;
                }
                visit[idx] = true;
                let g = bank[idx];
                if g == end {
                    return level;
                }
                for (bank_idx, b) in bank.iter().enumerate() {
                    if visit[bank_idx] {
                        continue;
                    }
                    if Self::is_mutable(g, *b) {
                        next_queue.push(bank_idx);
                    }
                }
            }
            level+=1;
            queue=next_queue;
        }

        -1
    }
    fn trans(s: String) -> [char;8] {
        let mut ret = ['a';8];
        for (idx, c) in s.chars().enumerate() {
            ret[idx] = c;
        }
        ret
    }
    fn is_mutable(left: [char;8], right: [char;8]) -> bool {
        let count = left.iter().zip(right.iter()).map(|x| *x.0 == *x.1).filter(|x| !*x).count();
        count==1
    }
}