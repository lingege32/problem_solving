use std::cmp::Reverse;
#[derive(Debug)]
struct MedianFinder {
    low_region: std::collections::BinaryHeap<i32>,
    high_region: std::collections::BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            low_region: Default::default(),
            high_region: Default::default(),
        }
    }

    #[allow(dead_code)]
    fn add_num(&mut self, num: i32) {
        let a = *self.low_region.peek().unwrap_or(&i32::MAX);
        if num > a {
            self.high_region.push(Reverse(num));
            if self.low_region.len() + 2 == self.high_region.len() {
                let top = self.high_region.pop().unwrap().0;
                self.low_region.push(top);
            }
        } else {
            self.low_region.push(num);
            if self.low_region.len() == self.high_region.len() + 2 {
                let top = self.low_region.pop().unwrap();
                self.high_region.push(Reverse(top));
            }
        }
    }

    #[allow(dead_code)]
    fn find_median(&self) -> f64 {
        let lmin = self.low_region.len();
        let rmin = self.high_region.len();
        if lmin == rmin {
            (*self.low_region.peek().unwrap() as f64 + self.high_region.peek().unwrap().0 as f64)
                / 2.0
        } else if lmin < rmin {
            self.high_region.peek().unwrap().0 as f64
        } else {
            *self.low_region.peek().unwrap() as f64
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = MedianFinder::new();
        a.add_num(1);
        a.add_num(2);
        a.add_num(3);
        println!("{:?}", a);
    }
}
