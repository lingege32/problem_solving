fn dbl_linear(n: u32) -> u32 {
    let (mut v, mut x, mut y) = (vec![1u32], 0, 0);
    while v.len() <= n as usize {
        let a = 2 * v[x] + 1;
        let b = 3 * v[y] + 1;
        if a < b {
            v.push(a);
            x += 1;
        } else if a > b {
            v.push(b);
            y += 1;
        } else {
            v.push(a);
            x += 1;
            y += 1;
        }
    }
    v[n as usize]
}
#[cfg(test)]
mod tests {
    use super::dbl_linear;
    fn testing(n: u32, exp: u32) -> () {
        assert_eq!(dbl_linear(n), exp)
    }

    #[test]
    fn basics_dbl_linear() {
        testing(10, 22);
        testing(20, 57);
        testing(30, 91);
        testing(50, 175);
        testing(100, 447);
    }
}
