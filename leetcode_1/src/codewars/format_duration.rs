///
/// more clear method
///
fn format_duration(seconds: u64) -> String {
    let y = seconds / 31536000;
    let d = seconds / 86400 % 365;
    let h = seconds / 3600 % 24;
    let m = seconds / 60 % 60;
    let s = seconds % 60;
    let time  = vec![(y, "year"), (d, "day"), (h, "hour"), (m, "minute"), (s, "second")];
    let str_v: Vec<_> = time.into_iter().filter(|t| t.0 != 0)
        .map(|(i, u)| format!("{} {}{}", i, u, if i == 1 { "" } else { "s" })).collect();
    match str_v.len() {
        0 => "now".into(),
        1 => str_v.into_iter().last().unwrap(),
        _ =>  {
            str_v.split_last().map(|(a,b)|  format!("{} and {}", b.join(", "), a)).unwrap()
            
        }
    }
}
///
/// My first implement
///
// fn format_duration(seconds: u64) -> String {
//     if seconds == 0 {
//         "now".to_string()
//     } else {
//         let v = vec![(60*60*24*365,"year"), (60*60*24, "day"), (60*60, "hour"), (60, "minute"), (1, "second")];
//         let mut seconds = seconds;
//         let mut vs = Vec::new();
//         for (unit, s) in v {
//             let (tmp, remainder) = helper_function(seconds, unit, s);
//             if let Some(sss) = tmp {
//                 vs.push(sss);
//                 seconds = remainder;
//             }
//         }
//         if vs.len() == 1 {
//             vs.pop().unwrap()
//         } else {
//             let last = vs.pop().unwrap();
//             format!("{} and {}", vs.join(", "), last)
//         }
//     }
    
// }
// fn helper_function(seconds: u64, unit: u64, unit_name: &str) -> (Option<String>, u64) {
//     if seconds < unit {
//         (None, seconds)
//     } else {
//         let (q, r) = (seconds/unit, seconds%unit);
//         match q {
//             1 => (Some(format!("1 {}", unit_name)), r),
//             n => (Some(format!("{} {}s",n, unit_name)), r)
//         }
//     }
// }


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}
