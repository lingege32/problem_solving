#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    hit: bool,
}

impl Trie {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            children: Default::default(),
            hit: false,
        }
    }
    #[allow(dead_code)]
    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.children[c as usize - 'a' as usize]
                .get_or_insert_with(|| Box::new(Trie::new()));
        }

        cur.hit = true;
    }
    #[allow(dead_code)]
    fn search(&self, word: String) -> bool {
        match self.find(&word) {
            Some(t) => t.hit,
            None => false,
        }
    }
    #[allow(dead_code)]
    fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix).is_some()
    }
    fn find(&self, word: &str) -> Option<&Trie> {
        let mut cur = self;
        for c in word.chars() {
            match &cur.children[c as usize - 'a' as usize] {
                Some(child) => {
                    cur = &child;
                }
                None => {
                    return None;
                }
            }
        }
        Some(cur)
    }
}

// #[derive(Default)]
// struct Trie {
//     chars: [Option<Box<Trie>>; 26],
//     end: bool,
// }

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl Trie {
//     fn new() -> Self {
//         Trie::default()
//     }
//     fn insert(&mut self, word: String) {
//         let mut node = self;
//         for c in word.chars() {
//             let k = c as usize - 'a' as usize;
//             node = node.chars[k].get_or_insert_with(|| Box::new(Trie::new()));
//         }
//         node.end = true;
//     }
//     fn search(&self, word: String) -> bool {
//         match self.find(&word) {
//             Some(node) => node.end,
//             None => false
//         }
//     }
//     fn starts_with(&self, prefix: String) -> bool {
//         self.find(&prefix).is_some()
//     }
//     fn find(&self, prefix: &str) -> Option<&Trie> {
//         let mut node = self;
//         for c in prefix.chars() {
//             let k = c as usize - 'a' as usize;
//             match &node.chars[k] {
//                 Some(next) => {
//                     node = &next;
//                 },
//                 None => {
//                     return None;
//                 }
//             }
//         }
//         return Some(node);
//     }
// }
