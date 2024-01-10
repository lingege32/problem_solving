#[derive(Default)]
struct Trie {
    hit: bool,
    children: [Option<Box<Trie>>; 26],
}
impl Trie {
    #[allow(dead_code)]
    fn new() -> Trie {
        Trie {
            hit: false,
            children: Default::default(),
        }
    }
    #[allow(dead_code)]
    fn add(&mut self, s: &str) {
        if s.is_empty() {
            self.hit = true;
            return;
        }
        let c = s.chars().next().unwrap() as usize - 'a' as usize;
        if self.children[c].is_none() {
            self.children[c] = Some(Box::new(Trie::new()));
        }
        self.children[c].as_mut().unwrap().add(&s[1..]);
    }
    #[allow(dead_code)]
    fn find(&self, s: &str) -> bool {
        if s.is_empty() {
            return self.hit;
        }
        let c = s.chars().next().unwrap();
        if c == '.' {
            self.children.iter().any(|x| match x {
                Some(next) => next.find(&s[1..]),
                None => false,
            })
        } else {
            match &self.children[c as usize - 'a' as usize] {
                Some(next) => next.find(&s[1..]),
                None => false,
            }
        }
    }
}

struct WordDictionary {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { trie: Trie::new() }
    }

    #[allow(dead_code)]
    fn add_word(&mut self, word: String) {
        self.trie.add(&word);
    }

    #[allow(dead_code)]
    fn search(&self, word: String) -> bool {
        self.trie.find(&word)
    }
}
