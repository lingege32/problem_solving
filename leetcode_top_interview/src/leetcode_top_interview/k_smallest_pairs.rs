struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut hs = std::collections::HashSet::<(usize, usize)>::new();
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        heap.push(Reverse((nums1[0] + nums2[0], 0, 0)));
        let mut ret = Vec::with_capacity(k as usize);
        while let Some(Reverse((_, i, j))) = heap.pop() {
            ret.push(vec![nums1[i], nums2[j]]);
            if ret.len() == k as usize {
                break;
            }

            if j + 1 < nums2.len() && hs.insert((i, j + 1)) {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
            if i + 1 < nums1.len() && hs.insert((i + 1, j)) {
                heap.push(Reverse((nums1[i + 1] + nums2[j], i + 1, j)));
            }
        }
        ret
    }
}
