use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
struct MedianFinder {
    l: BinaryHeap<i32>,
    r: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    #[allow(dead_code)]
    fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    fn add_num(&mut self, num: i32) {
        // println!("self.r.peek(): {:?}, num: {}", self.r.peek(), num);
        match self.r.peek() {
            None => self.r.push(Reverse(num)),
            Some(Reverse(v)) if num > *v => self.r.push(Reverse(num)),
            _ => self.l.push(num),
        }

        // maintain balance
        if self.r.len() > self.l.len() + 1 {
            self.l.push(self.r.pop().unwrap().0);
        } else if self.l.len() > self.r.len() + 1 {
            self.r.push(Reverse(self.l.pop().unwrap()))
        }
    }

    #[allow(dead_code)]
    fn find_median(&self) -> f64 {
        // println!("l: {:?}", self.l);
        // println!("r: {:?}", self.r);
        return if self.l.len() > self.r.len() {
            *(self.l.peek().unwrap()) as f64
        } else if self.r.len() > self.l.len() {
            self.r.peek().unwrap().0 as f64
        } else {
            let l = *(self.l.peek().unwrap()) as f64;
            let r = self.r.peek().unwrap().0 as f64;
            (l + r) / 2.0
        };
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

// garbage algorithm, only beat 9%
// struct MedianFinder {
//     inner: Vec<i32>,
// }
// impl MedianFinder {
//     #[allow(dead_code)]
//     fn new() -> Self {
//         Self { inner: Vec::new() }
//     }
//     #[allow(dead_code)]
//     fn add_num(&mut self, num: i32) {
//         let (mut left, mut right) = (0usize, self.inner.len());
//         while left < right {
//             let mid = (left+right)/2;
//             if self.inner[mid] < num {
//                 left = mid+1;
//             } else {
//                 right = mid;
//             }
//         }
//         self.inner.insert(left, num);
//         println!("self.inner: {:?}", self.inner);
//     }
//     #[allow(dead_code)]
//     fn find_median(&self) -> f64 {
//         let len = self.inner.len();
//         let mid = len / 2;
//         if len % 2 == 1 {
//             self.inner[mid] as f64
//         } else {
//             (self.inner[mid] + self.inner[mid - 1]) as f64 / 2f64
//         }
//     }
// }

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(8);
        mf.add_num(4);
        mf.add_num(7);
        mf.add_num(2);
        mf.add_num(3);
        mf.add_num(3);
        mf.add_num(7);
        mf.find_median();
    }
}
