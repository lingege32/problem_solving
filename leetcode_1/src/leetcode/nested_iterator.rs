#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}
struct NestedIterator {
    stack: Vec<NestedInteger>,
    next: Option<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut s = Self {
            stack: nested_list.into_iter().rev().collect::<Vec<_>>(),
            next: None,
        };
        s.advance_next();
        s
    }
    
    fn next(&mut self) -> i32 {
        let ret = self.next.unwrap();
        self.advance_next();
        ret
    }
    
    fn has_next(&self) -> bool {
        self.next.is_some()
    }

    fn advance_next(&mut self) {
        while let Some(last) = self.stack.pop() {
            match last {
                NestedInteger::Int(n) => {
                    self.next = Some(n);
                    return;
                }
                NestedInteger::List(list) => {
                    self.stack.extend(list.into_iter().rev());
                }
            }
        }
        self.next = None;
    }
}