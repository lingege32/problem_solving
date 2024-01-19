struct Solution;
use std::iter::FromIterator;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let end_word = end_word.as_bytes().to_owned();
        let begin_word = begin_word.as_bytes().to_owned();

        let mut word_list: HashSet<Vec<u8>> =
            HashSet::from_iter(word_list.iter().map(|e| e.as_bytes().to_owned()));
        if word_list.get(&end_word).is_none() {
            return 0;
        }

        word_list.remove(&begin_word);
        word_list.remove(&end_word);

        let mut q1 = vec![begin_word.clone()];
        let mut q2 = vec![end_word.clone()];
        q1.reserve(word_list.len() / 2);
        q2.reserve(word_list.len() / 2);

        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        m1.insert(begin_word, 1);
        m2.insert(end_word, 1);

        let mut qindex1 = 0;
        let mut qindex2 = 0;

        let mut word_count1 = 1;
        let mut word_count2 = 1;

        while qindex1 != q1.len() && qindex2 != q2.len() {
            if qindex1 <= qindex2 {
                word_count1 += 1;

                for _ in qindex1..q1.len() {
                    let mut cur = Vec::new();
                    std::mem::swap(&mut cur, &mut q1[qindex1]);
                    qindex1 += 1;

                    for i in 0..cur.len() {
                        let prev = cur[i];
                        for chr in b'a'..=b'z' {
                            cur[i] = chr;

                            if m2.get(&cur).is_some() {
                                let m2d = m2.get(&cur).unwrap();
                                cur[i] = prev;
                                let m1d = m1.get(&cur).unwrap();
                                return m1d + m2d;
                            }

                            if let Some(new_word) = word_list.take(&cur) {
                                q1.push(new_word.clone());
                                m1.insert(new_word, word_count1);
                            }
                        }
                        cur[i] = prev;
                    }
                }
            } else {
                word_count2 += 1;

                for _ in qindex2..q2.len() {
                    let mut cur = Vec::new();
                    std::mem::swap(&mut cur, &mut q2[qindex2]);
                    qindex2 += 1;

                    for i in 0..cur.len() {
                        let prev = cur[i];
                        for chr in b'a'..=b'z' {
                            cur[i] = chr;

                            if m1.get(&cur).is_some() {
                                let m1d = m1.get(&cur).unwrap();
                                cur[i] = prev;
                                let m2d = m2.get(&cur).unwrap();
                                return m1d + m2d;
                            }

                            if let Some(new_word) = word_list.take(&cur) {
                                q2.push(new_word.clone());
                                m2.insert(new_word, word_count2);
                            }
                        }
                        cur[i] = prev;
                    }
                }
            }
        }

        return 0;
    }
}
// impl Solution {
//     #[allow(dead_code)]
//     pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
//         let mut word_list = word_list
//             .into_iter()
//             .map(|x| (false, x.into_bytes()))
//             .collect::<Vec<_>>();
//         let begin = begin_word.into_bytes();
//         let end = end_word.into_bytes();
//         word_list.push((false, begin.clone()));
//         let mut queue= vec![word_list.len() - 1];
//         let mut dis = 1;
//         while !queue.is_empty() {
//             let mut next_queue = Vec::new();
//             for idx in queue {
//                 if word_list[idx].0 {
//                     continue;
//                 }
//                 word_list[idx].0 = true;
//                 let name = &word_list[idx].1;
//                 if name == &end {
//                     return dis;
//                 }

//                 for (next_idx, (visit, next)) in word_list.iter().enumerate() {
//                     if *visit {
//                         continue;
//                     }
//                     if Self::is_transofmed(next, &name) {
//                         next_queue.push(next_idx);
//                     }
//                 }
//             }
//             dis += 1;
//             queue = next_queue;
//         }

//         0
//     }

//     fn is_transofmed(l: &[u8], r: &[u8]) -> bool {
//         l.iter().zip(r.iter()).filter(|(x, y)| x != y).count() == 1
//     }
// }
