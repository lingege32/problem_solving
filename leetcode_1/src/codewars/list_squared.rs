fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..=n).map(|x| (x, sum_of_factor_squares(x))).filter(|(_,x)|  (*x as f64).sqrt().fract() == 0.0).collect()
}
fn sum_of_factor_squares(m: u64) -> u64 {

    (1..=m).filter(|&x| m%(x)==0 && x*x <= m).map(|x| {let u = m/x; if u==x {x*x} else {u*u + x*x}}).sum()
}


fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
fn basics_list_squared() {

    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);
    
}