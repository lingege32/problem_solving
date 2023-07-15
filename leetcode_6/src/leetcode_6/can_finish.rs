struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut how_many_index = vec![0; num_courses];
        let mut to_node = vec![Vec::<usize>::new(); num_courses];
        for pre in prerequisites {
            let (c, p) = (pre[0] as usize, pre[1] as usize);
            how_many_index[c] += 1;
            to_node[p].push(c);
        }
        let mut queue = std::collections::VecDeque::new();
        for (idx, nodes) in how_many_index.iter().enumerate() {
            if *nodes == 0 {
                queue.push_back(idx);
            }
        }
        let mut finish = 0;
        while let Some(course) = queue.pop_front() {
            finish += 1;
            for &next_course in to_node[course].iter() {
                let n = unsafe { how_many_index.get_unchecked_mut(next_course) };
                *n -= 1;
                if *n == 0 {
                    queue.push_back(next_course);
                }
            }
        }
        finish == num_courses
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let num_courses = 2;
        let prerequisites = [[1, 0]].iter().map(|x| x.to_vec()).collect::<Vec<_>>();
        let ans = true;
        assert_eq!(ans, Solution::can_finish(num_courses, prerequisites));
    }
    #[test]
    fn test_2() {
        let num_courses = 2;
        let prerequisites = [[1, 0], [0, 1]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let ans = false;
        assert_eq!(ans, Solution::can_finish(num_courses, prerequisites));
    }
    #[test]
    fn test_3() {
        let num_courses = 5;
        let prerequisites = [[1, 4], [2, 4], [3, 1], [3, 2]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        let ans = true;
        assert_eq!(ans, Solution::can_finish(num_courses, prerequisites));
    }
}
