use rand::seq::SliceRandom;
use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::<i32, usize>::new(),
            v: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let idx = *self.map.entry(val).or_insert(self.v.len());
        if idx == self.v.len() {
            self.v.push(val);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(idx) = self.map.remove(&val) {
            let last = *self.v.last().unwrap();
            if let Some(last_idx) = self.map.get_mut(&last) {
                *last_idx = idx;
            }
            self.v.swap_remove(idx);
            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        *self.v.choose(&mut rand::thread_rng()).unwrap()
    }
}
