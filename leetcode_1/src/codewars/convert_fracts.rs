// other's simplfy cocde

// fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b)} }
// fn lcm(a: i64, b: i64) -> i64 { a / gcd(a, b) * b }
// fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
//     let d = l.iter().fold(1, |acc, &(num, den)| lcm(acc, den/gcd(num, den)));
//     l.iter().map(|&(num, den)| (num*d/den, d)).collect()
// }


fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // your code
    let mut l = l ;
    for (numerator, denominator) in l.iter_mut() {
        let gcf = greatest_common_factor(*numerator, *denominator);
        (*numerator) /= gcf;
        (*denominator) /= gcf;
    }
    let lcm = l.iter().map(|(_,c)| c).fold(1,|acc,b| (least_common_multiple(acc, *b)));
    for (numerator, denominator) in l.iter_mut() {
        *numerator = *numerator * (lcm / *denominator);
        *denominator = lcm;
    }
    l
}
fn least_common_multiple(a: i64, b: i64) -> i64{
    a * b / greatest_common_factor(a, b)
}
fn greatest_common_factor(a: i64, b: i64) -> i64{
    if a == 0 {
        b
    } else if a>b {
        greatest_common_factor(b, a)
    } else {
        greatest_common_factor(b%a, a)
    }
}

fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(convert_fracts(l), exp)
}

#[test]
fn basics_convert_fracts() {
	
    testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);

}