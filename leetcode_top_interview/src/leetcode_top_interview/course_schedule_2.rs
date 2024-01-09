struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        // pre --> cour
        let mut courses_next = vec![vec![]; num_courses as usize];
        let mut inedges = vec![0; num_courses as usize];
        for v in prerequisites.into_iter() {
            let (c, p) = (v[0], v[1]);
            courses_next[p as usize].push(c as usize);
            inedges[c as usize] += 1;
        }
        let mut stack = vec![];
        for i in 0..num_courses as usize {
            if inedges[i] == 0 {
                stack.push(i);
            }
        }
        let mut ret = vec![];
        while let Some(c) = stack.pop() {
            ret.push(c as i32);
            for &next in courses_next[c].iter() {
                inedges[next] -= 1;
                if inedges[next] == 0 {
                    stack.push(next);
                }
            }
        }

        if ret.len() == num_courses as usize {
            ret
        } else {
            vec![]
        }
    }
}
