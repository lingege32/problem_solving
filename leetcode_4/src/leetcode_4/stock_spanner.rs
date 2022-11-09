struct StockSpanner {
    m: Vec<(usize, i32)>,
    cur_idx: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    #[allow(dead_code)]
    fn new() -> Self {
        StockSpanner {
            m: Vec::new(),
            cur_idx: 0,
        }
    }
    
    #[allow(dead_code)]
    fn next(&mut self, price: i32) -> i32 {
        let local_idx = &mut self.cur_idx;
        let local_m = &mut self.m;
        *local_idx += 1;
        while !local_m.is_empty() && local_m.last().unwrap().1 <= price {
            local_m.pop();
        }
        
        let ret = local_m.last().map(|x| x.0).unwrap_or(0);
        local_m.push((*local_idx, price));
        (*local_idx - ret) as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = Vec::<i32>::new();
        let mut stock_spanner = StockSpanner::new();
        v.push(stock_spanner.next(100)); // return 1
        v.push(stock_spanner.next(80)); // return 1
        v.push(stock_spanner.next(60)); // return 1
        v.push(stock_spanner.next(70)); // return 2
        v.push(stock_spanner.next(60)); // return 1
        v.push(stock_spanner.next(75)); // return 4, because the last 4 prices (including today's price of 75) were less than or equal to today's price.
        v.push(stock_spanner.next(85)); // return 6
        assert_eq!(v, vec![1, 1, 1, 2, 1, 4, 6]);
    }
}
