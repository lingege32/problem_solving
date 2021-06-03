use std::collections::BTreeMap;
fn prime_factors(n: i64) -> String {
    // your code
    let mut map = BTreeMap::new();
    recr_prime_factors(n, 2, &mut map);
    map.into_iter().map(|s| if s.1 > 1 {format!("({}**{})", s.0, s.1)} else {format!("({})",s.0)}).collect::<String>()
}

fn recr_prime_factors(n:i64, start: i64, map: &mut BTreeMap<i64, i64>) {
    for i in start.. {
        if i*i > n {
            break;
        }
        if n%i == 0 {
            *map.entry(i).or_insert(0)+=1;
            recr_prime_factors(n/i, i, map);
            return;
            
        }
    }
    *map.entry(n).or_insert(0)+=1;
}




fn testing(n: i64, exp: &str) -> () {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn basics_prime_factors() {
    
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17*17*93*677, "(3)(17**2)(31)(677)");
    
}