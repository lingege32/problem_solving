fn solution(s: &str) -> Vec<String> {
    let mut v = Vec::with_capacity(s.len()/2+1);
    let mut c = s.chars();
    for _ in (0..s.len()).step_by(2) {
        let mut st = String::new();
        st.push(c.next().unwrap());
        st.push(c.next().unwrap_or('_'));
        v.push(st);
    }
    v
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
