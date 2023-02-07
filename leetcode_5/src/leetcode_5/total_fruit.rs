struct Solution;
use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut hm = HashMap::new();

        let mut left = 0;
        for right in 0..fruits.len() {
            *hm.entry(fruits[right]).or_insert(0) += 1;
            while hm.len() > 2 {
                let fruit_type = fruits[left];
                let v = hm.get_mut(&fruit_type).unwrap();
                *v -= 1;
                if *v == 0 {
                    hm.remove(&fruit_type);
                }
                left += 1;
            }
            ans = ans.max((right-left+1) as i32);
        }
        ans
    }

    // The idea is same with me, but I use hashmap and the performance is not good
    
    #[allow(dead_code)]
    pub fn better_total_fruit(fruits: Vec<i32>) -> i32 {
        use std::cmp::{max,min};
        
        let mut start_first: usize = 0;
        let mut start_second: usize = 0;
        let mut end_first: usize = 0;
        let mut end_second: usize = 0;
        let mut first_bag: Option<i32> = None;
        let mut second_bag: Option<i32> = None;
        let mut max_q: usize = 0;


        for (i, &fruit) in fruits.iter().enumerate() {
            if first_bag.is_none() {
                first_bag = Some(fruit);
                start_first = i;
                end_first = i + 1;
            } else if first_bag.unwrap() == fruit {
                end_first = i + 1;
            } else if second_bag.is_none() {
                second_bag = Some(fruit);
                start_second = i;
                end_second = i + 1;
            } else if second_bag.unwrap() == fruit {
                end_second = i + 1;
            } else {
                max_q = max(max_q, max(end_second, end_first) - min(start_first, start_second));
                if end_second > end_first {
                    start_second = end_first;

                    end_first = i + 1;
                    first_bag = Some(fruit);
                    start_first = i;
                } else {
                    start_first = end_second;

                    second_bag = Some(fruit);
                    start_second = i;
                    end_second = i + 1;
                }
            }
        }
        max(max_q, max(end_second, end_first) - min(start_first, start_second)) as i32
    }
}