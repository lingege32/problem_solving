



// itertools solution
//
// use itertools::Itertools;
// fn anagrams(word: &str, words: &[String]) -> Vec<String> {
//     let cs = word.chars().sorted().collect_vec();
//     words
//         .iter()
//         .filter(|s| s.chars().sorted().collect_vec() == cs)
//         .cloned()
//         .collect()
// }

// my solution

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut a = word.as_bytes().to_owned();
    a.sort();
    words.iter()
    .filter(|&s| {
        if s.len() != a.len() {
            false
        } else {
            let mut t = s.as_bytes().to_owned();
            t.sort();
            t == a
        }
    })
    .cloned()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);

        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}
