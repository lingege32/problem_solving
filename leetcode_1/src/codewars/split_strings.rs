fn solution(s: &str) -> Vec<String> {
    let vec = s
    .as_bytes()
    .chunks(2)
    .map(|x| format!("{:_<2}", std::str::from_utf8(x).unwrap())) // format!("{:_<2}") 印出兩個字，不足的用_代替
    .collect();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
