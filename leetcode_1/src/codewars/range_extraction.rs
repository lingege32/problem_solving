mod solution {
    
    // my new solution
    use itertools::Itertools;
    pub fn range_extraction(a: &[i32]) -> String {
        itertools::join(
            (0..a.len())
                .group_by(|&i| (a[i]).wrapping_sub(i as i32))
                .into_iter()
                .map(|(_, v)| v.map(|i| a[i]).collect::<Vec<i32>>())
                .map(|v| match v.len() {
                    1 => v[0].to_string(),
                    2 => format!("{},{}", v[0], v[1]),
                    _ => format!("{}-{}", v[0], v.last().unwrap()),
                }),
            ",",
        )
    }


    // My original solution
    //
    // #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    // struct Range {
    //     begin: i32,
    //     dis: i32,
    // }
    // impl Range {
    //     pub fn new(i: i32) -> Self {
    //         Range { begin: i, dis: 1 }
    //     }
    // }
    // impl ToString for Range {
    //     fn to_string(&self) -> String {
    //         if self.dis == 1 {
    //             format!("{}", self.begin)
    //         } else if self.dis==2 {
    //             format!("{},{}", self.begin, self.begin - 1 + self.dis)
    //         } else {
    //             format!("{}-{}", self.begin, self.begin - 1 + self.dis)
    //         }
    //     }
    // }
    // pub fn range_extraction(a: &[i32]) -> String {
    //     if a.is_empty() {
    //         "".to_string()
    //     } else {
    //         let mut vr = Vec::new();
    //         let mut rg = Range::new(a[0]);
    //         for &val in &a[1..] {
    //             if rg.begin + rg.dis == val {
    //                 rg.dis += 1;
    //             } else {
    //                 vr.push(rg);
    //                 rg = Range::new(val);
    //             }
    //         }
    //         vr.push(rg);
    //         itertools::join(vr.into_iter().map(|r| r.to_string()), ",")
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            "-6,-3-1,3-5,7-11,14,15,17-20",
            solution::range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ])
        );
        assert_eq!(
            "-3--1,2,10,15,16,18-20",
            solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20])
        );
    }
}
