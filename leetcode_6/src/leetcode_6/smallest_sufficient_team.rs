struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let (m, n) = (req_skills.len(), people.len());
        let mut skill_to_i = std::collections::HashMap::with_capacity(m);
        for (i, skill) in req_skills.into_iter().enumerate() {
            skill_to_i.insert(skill, i);
        }
        let people = people.into_iter().enumerate().map(|(_i, skills)| skills.into_iter().fold(0, |bm, skill| bm | 1 << skill_to_i.get(&skill).unwrap())).collect::<Vec<_>>();
        let mut dp = vec![(n,0,0);1<<m];
        dp[0].0 = 0;
        for (i, person) in people.into_iter().enumerate() {
            if person == 0 { continue }
            for skills in 0..1<<m {
                let req_skills = skills | person;
                if dp[skills].0 + 1 < dp[req_skills].0 {
                    dp[req_skills].0 = dp[skills].0 + 1;
                    dp[req_skills].1 = skills;
                    dp[req_skills].2 = i as i32;
                }
            }
        }
        let mut i = (1<<m)-1;
        let mut team = Vec::new();
        while i != 0 {
            team.push(dp[i].2);
            i = dp[i].1;
        }
        team
    }
}
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        let req_skills = ["java", "nodejs", "reactjs"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let people = [vec!["java"], vec!["nodejs"], vec!["nodejs", "reactjs"]]
            .iter()
            .map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let ans = vec![0, 2];
        assert_eq!(Solution::smallest_sufficient_team(req_skills, people), ans);
    }
    #[test]
    fn test_2() {
        let req_skills = ["algorithms", "math", "java", "reactjs", "csharp", "aws"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let people = [
            vec!["algorithms", "math", "java"],
            vec!["algorithms", "math", "reactjs"],
            vec!["java", "csharp", "aws"],
            vec!["reactjs", "csharp"],
            vec!["csharp", "math"],
            vec!["aws", "java"],
        ]
        .iter()
        .map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
        let ans = vec![1, 2];
        assert_eq!(Solution::smallest_sufficient_team(req_skills, people), ans);
    }
}
