struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut v = vec![Vec::new(); n as usize];
        trust.iter().for_each(|x| {
            v[x[1] as usize - 1].push(x[0] - 1);
        });
        let a = v.iter().enumerate().find(|&x| x.1.len() == n as usize - 1);
        match a {
            Some((n, _)) => {
                let find = trust.into_iter().find(|x| x[0] == n as i32 + 1);
                match find {
                    Some(_) => -1,
                    None => n as i32 + 1,
                }
            }
            None => -1,
        }
    }
}

// another Solution
// impl Solution {
//     pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
//         let n = n as usize;
//         let mut be_trusted = vec![0; n + 1];
//         let mut non_judge = vec![false; n + 1];

//         for pair in trust {
//             be_trusted[pair[1] as usize] += 1;
//             non_judge[pair[0] as usize] = true;
//         }
//         for i in 1..=n {
//             if be_trusted[i] == n - 1 && !non_judge[i] {
//                 return i as i32;
//             }
//         }
//         -1
//     }
// }
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let trust = [[1, 2]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(2, Solution::find_judge(n, trust));
    }

    #[test]
    fn test_23() {
        let n = 3;
        let trust = [[1, 3], [2, 3]].iter().map(|x| x.to_vec()).collect();
        assert_eq!(3, Solution::find_judge(n, trust));
    }

    #[test]
    fn test_12() {
        let n = 3;
        let trust = [[1, 2], [2, 3], [3, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(-1, Solution::find_judge(n, trust));
    }
}
