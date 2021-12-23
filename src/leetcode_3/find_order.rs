struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut conflict_table = vec![Vec::new(); num_courses as usize];
        let mut how_many_conflict = vec![0; num_courses as usize];
        for pre in prerequisites {
            how_many_conflict[pre[0] as usize] += 1;
            conflict_table[pre[1] as usize].push(pre[0] as usize);
        }

        let mut no_prerequisites = Vec::new();
        for idx in 0..num_courses as usize {
            if how_many_conflict[idx] == 0 {
                no_prerequisites.push(idx as i32);
            }
        }
        let (mut begin, mut end) = (0, no_prerequisites.len());
        while no_prerequisites.len() != num_courses as usize {
            for idx in begin..end {
                let course = no_prerequisites[idx] as usize;
                for prerequire_to_course in conflict_table[course].iter() {
                    how_many_conflict[*prerequire_to_course] -= 1;
                    if how_many_conflict[*prerequire_to_course] == 0 {
                        no_prerequisites.push(*prerequire_to_course as i32);
                    }
                }
            }
            if end == no_prerequisites.len() {
                return vec![];
            }
            begin = end;
            end = no_prerequisites.len();
        }

        no_prerequisites
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let num_courses = 2;
        let prerequisites = [[1, 0]].iter().map(|x| x.to_vec()).collect::<Vec<_>>();
        assert_eq!(vec![0, 1], Solution::find_order(num_courses, prerequisites));
    }

    #[test]
    fn test_2() {
        let num_courses = 4;
        let prerequisites = [[1, 0], [2, 0], [3, 1], [3, 2]]
            .iter()
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(
            vec![0, 1, 2, 3],
            Solution::find_order(num_courses, prerequisites)
        );
    }
}
