struct MinStack {
    data: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { data: vec![] }
    }
    #[allow(dead_code)]
    fn push(&mut self, val: i32) {
        if self.data.is_empty() {
            self.data.push((val, val));
        } else {
            let min = self.get_min();
            self.data.push((val, val.min(min)));
        }
    }
    #[allow(dead_code)]
    fn pop(&mut self) {
        self.data.pop();
    }
    #[allow(dead_code)]
    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }
    #[allow(dead_code)]
    fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}
