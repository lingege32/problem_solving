// Other's more abstract code
// struct RaceTrack(Vec<i32>);

// impl RaceTrack {
//     fn new_contestant(&self) -> Contestant {
//         Contestant {
//             race_track: self,
//             position: 0,
//         }
//     }
// }

// struct Contestant<'a> {
//     race_track: &'a RaceTrack,
//     position: usize,
// }

// impl<'a> Contestant<'a> {
//     fn next(&mut self) {
//         self.position = self.race_track.0[self.position] as usize;
//     }
// }

// impl Solution {
//     pub fn find_duplicate(nums: Vec<i32>) -> i32 {
//         let race_track = RaceTrack(nums);
//         let (mut turtle, mut rabbit) = (race_track.new_contestant(), race_track.new_contestant());
//         loop {
//             turtle.next();
//             rabbit.next();
//             rabbit.next();
//             if turtle.position == rabbit.position {
//                 let mut snail = race_track.new_contestant();
//                 while turtle.position != snail.position {
//                     turtle.next();
//                     snail.next();
//                 }
//                 return snail.position as i32;
//             }
//         }
//         unreachable!();
//     }
// }



struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }
        fast = 0;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        slow as i32
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_duplicate(vec![1, 1]), 1);
    }
}
