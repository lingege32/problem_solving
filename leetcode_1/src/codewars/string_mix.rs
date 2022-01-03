fn mix(s1: &str, s2: &str) -> String {
    let mut vec = ('a'..='z')
        .map(|x| (x, 0, 0))
        .collect::<Vec<(char, usize, usize)>>();
    for c in s1.chars().filter(|x| match x {
        'a'..='z' => true,
        _ => false,
    }) {
        vec[(c as u8 - 'a' as u8) as usize].1 += 1;
    }
    for c in s2.chars().filter(|x| match x {
        'a'..='z' => true,
        _ => false,
    }) {
        vec[(c as u8 - 'a' as u8) as usize].2 += 1;
    }
    vec.iter_mut().for_each(|(_, s1, s2)| {
        match s1.cmp(&s2) {
            std::cmp::Ordering::Equal => *s2 = 3,
            std::cmp::Ordering::Less => {
                *s1 = *s2;
                *s2 = 2;
            },
            std::cmp::Ordering::Greater => *s2 = 1,
        }
    });
    vec.sort_by(|(_, llen, lmask), (_, rlen, rmask)| match rlen.cmp(llen) {
        std::cmp::Ordering::Equal => lmask.cmp(rmask),
        a => a,
    });
    vec.iter()
        .take_while(|(_, len, _)| *len > 1)
        .map(|(s, len, mask)| {
            format!(
                "{}:{}",
                if *mask == 3 {
                    "=".to_string()
                } else {
                    mask.to_string()
                },
                std::iter::repeat(s).take(*len).collect::<String>()
            )
        })
        .collect::<Vec<String>>()
        .join("/")
}
#[cfg(test)]
mod tests {
    use super::mix;

    #[test]
    fn basics_mix() {
        testing(
            "Are they here",
            "yes, they are here",
            "2:eeeee/2:yy/=:hh/=:rr",
        );
        testing(
            "looping is fun but dangerous",
            "less dangerous than coding",
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
        );
        testing(
            " In many languages",
            " there's a pair of functions",
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt",
        );
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing(
            "A generation must confront the looming ",
            "codewarrs",
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr",
        );
    }

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }
}
