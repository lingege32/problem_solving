struct MinStack {
    data : Vec<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            data: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        if self.data.is_empty() {
            self.data.push((val, val));
        } else {
            self.data.push((val, self.get_min().min(val)));
        }
    }
    
    fn pop(&mut self) {
        self.data.pop();
    }
    
    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}
