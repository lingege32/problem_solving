struct Solution;
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    str: String,
}
impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: Default::default(),
            str: String::new(),
        }
    }
    pub fn search(
        &mut self,
        board: &Vec<Vec<char>>,
        visit: &mut Vec<Vec<bool>>,
        index: (usize, usize),
        ret: &mut Vec<String>,
    ) {
        let (x, y) = index;
        let visited = unsafe { visit.get_unchecked_mut(x).get_unchecked_mut(y) };
        if *visited {
            return;
        }
        *visited = true;
        
        if self.str != "" {
            let mut tmp = String::new();
            std::mem::swap(&mut tmp, &mut self.str);
            ret.push(tmp);
        }
        let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for d in dir {
            let (mut x, mut y) = (x as i32, y as i32);
            x += d.0;
            y += d.1;
            if x >= 0
                && y >= 0
                && x < board.len() as i32
                && y < unsafe { board.get_unchecked(0).len() as i32 }
            {
                let trie_index = Self::char_to_idx(unsafe {
                    *board.get_unchecked(x as usize).get_unchecked(y as usize)
                });
                if let Some(child) = unsafe { self.children.get_unchecked_mut(trie_index) } {
                    child.search(board, visit, (x as usize, y as usize), ret);
                }
            }
        }
        let visited = unsafe { visit.get_unchecked_mut(x).get_unchecked_mut(y) };
        *visited = false;
    }
    fn char_to_idx(c: char) -> usize {
        c as usize - 'a' as usize
    }
}
struct TrieTree {
    root: TrieNode,
}

impl TrieTree {
    pub fn insert(&mut self, str: String) {
        let mut node = &mut self.root;
        for ch in str.chars() {
            let idx = Self::char_to_idx(ch);
            let a = unsafe { node.children.get_unchecked_mut(idx) };
            if a.is_none() {
                *a = Some(Box::new(TrieNode::new()));
            }
            node = a.as_mut().unwrap().as_mut();
        }
        node.str = str;
    }
    pub fn search(&mut self, board: &Vec<Vec<char>>) -> Vec<String> {
        let mut ret = Vec::new();
        let mut visit = vec![vec![false; board[0].len()]; board.len()];
        for row_idx in 0..board.len() {
            for col_idx in 0..board[0].len() {
                let idx = Self::char_to_idx(unsafe {
                    *board.get_unchecked(row_idx).get_unchecked(col_idx)
                });
                if let Some(child) = unsafe { self.root.children.get_unchecked_mut(idx) } {
                    child.search(board, &mut visit, (row_idx, col_idx), &mut ret);
                }
            }
        }
        ret
    }

    fn char_to_idx(c: char) -> usize {
        c as usize - 'a' as usize
    }
}
impl Solution {
    #[allow(dead_code)]
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trietree = TrieTree {
            root: TrieNode::new(),
        };
        for word in words {
            trietree.insert(word);
        }
        trietree.search(&board)
    }
}