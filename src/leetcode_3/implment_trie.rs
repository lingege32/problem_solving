/*
    more efficency algorithm
*/
// struct Trie {}
// static mut v: Vec<([usize; 26], bool)> = vec![];

// impl Trie {
//     fn new() -> Self {
//         unsafe { v = vec![([0; 26], false)]; }
//         Self {}
//     }
//     fn insert(&mut self, word: String) { unsafe {
//         println!("insert: {}", word);
//         let mut i = 0;
//         for c in word.bytes() {
//             println!("c: {}, i: {}, v: {:?}", c, i, v);
//             let d = (c - b'a') as usize;
//             if v[i].0[d] == 0 {
//                 v[i].0[d] = v.len();
//                 v.push(([0; 26], false));
//             }
//             i = v[i].0[d];
//         }
//         v[i].1 = true;
//     }}
//     fn __search(&mut self, word: String, cond: bool) -> bool { unsafe {
//         let mut i = 0;
//         for c in word.bytes() {
//             let d = (c - b'a') as usize;
//             if v[i].0[d] == 0 { return false; }
//             i = v[i].0[d];
//         }
//         return cond || v[i].1;
//     }}
//     fn search(&mut self, word: String) -> bool  { return self.__search(word, false); }
//     fn starts_with(&mut self, word: String) -> bool { return self.__search(word, true); }
// }

/**
* Your Trie object will be instantiated and called as such:
* let obj = Trie::new();
* obj.insert(word);
* let ret_2: bool = obj.search(word);
* let ret_3: bool = obj.starts_with(prefix);
*/
struct TrieNode {
    next: [Option<Box<TrieNode>>; 26],
    last: bool,
}
impl TrieNode {
    fn new() -> Self {
        TrieNode {
            next: Default::default(),
            last: false,
        }
    }
    fn set_last(&mut self) {
        self.last = true;
    }
    fn is_last(&self) -> bool {
        self.last
    }
}
struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    #[allow(dead_code)]
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    #[allow(dead_code)]
    fn insert(&mut self, word: String) {
        let mut r = &mut self.root;
        for idx in word.as_bytes().iter().map(|x| (*x - b'a') as usize) {
            r = r.next[idx].get_or_insert(Box::new(TrieNode::new()));
        }
        r.set_last();
    }

    #[allow(dead_code)]
    fn search(&self, word: String) -> bool {
        let mut r = &self.root;
        for idx in word.as_bytes().iter().map(|x| (*x - b'a') as usize) {
            match r.next[idx] {
                Some(ref n) => {
                    r = n;
                }
                None => {
                    return false;
                }
            }
        }
        r.is_last()
    }

    #[allow(dead_code)]
    fn starts_with(&self, prefix: String) -> bool {
        let mut r = &self.root;
        for idx in prefix.as_bytes().iter().map(|x| (*x - b'a') as usize) {
            match r.next[idx] {
                Some(ref n) => {
                    r = n;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}
