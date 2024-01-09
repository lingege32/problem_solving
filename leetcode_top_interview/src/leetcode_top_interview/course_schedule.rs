struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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
            if inedges[i] == 0 && !courses_next[i].is_empty() {
                stack.push(i);
            }
        }
        while let Some(c) = stack.pop() {
            for &next in courses_next[c].iter() {
                inedges[next] -= 1;
                if inedges[next] == 0 {
                    stack.push(next);
                }
            }
        }

        inedges.into_iter().all(|x| x == 0)
    }
}
