use std::collections::HashMap;
fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
    let mut hs = HashMap::<i64, i64>::new();
    for i in start..=limit {
        let c = complentary_num(i, &mut hs);
        let d = complentary_num(c - 1, &mut hs);

        if i == (d - 1) && i <= c-1{
            return Some((i, c - 1));
        }
    }
    None
}
fn complentary_num(i: i64, hs: &mut HashMap<i64, i64>) -> i64 {
    *hs.entry(i).or_insert_with(|| {
        (2..i)
            .take_while(|&x| x * x <= i)
            .filter(|&x| i % x == 0)
            .map(|x| {
                let x2 = i / x;
                if x == x2 {
                    x
                } else {
                    x + x2
                }
            })
            .sum::<i64>()
            + 1
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(start: i64, limit: i64, exp: Option<(i64, i64)>) -> () {
        println!("start:{}", start);
        println!("limit:{}", limit);
        let ans = buddy(start, limit);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10, 50, Some((48, 75)));
        dotest(1081180, 1103735, Some((1081184, 1331967)));
        dotest(544020, 546082, None);
    }
}
