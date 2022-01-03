fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if 0f64 < h && bounce > 0f64 && bounce < 1f64 && window < h {
        let mut count = 1;
        let mut h = h;
        loop {
            h *= bounce;
            if h <= window {
                break;
            }
            count += 2;
        }
        count
    } else {
        -1
    }
}

// Other's solution
// fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
//     if !(h > 0. && 0. < bounce && bounce < 1. && window < h) {
//         -1
//     } else {
//         (window / h).log(bounce).ceil() as i32 * 2 - 1
//     }
// }

fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

#[test]
fn tests_bouncing_ball() {
    testequal(3.0, 0.66, 1.5, 3);
    testequal(30.0, 0.66, 1.5, 15);
    testequal(40.0, 0.4, 10.0, 3);
    testequal(10.0, 0.6, 10.0, -1);
}
