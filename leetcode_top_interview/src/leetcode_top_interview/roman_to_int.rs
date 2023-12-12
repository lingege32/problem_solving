struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let v = s.into_bytes();
        let mut complex = false;
        let mut ret = 0;
        for window in v.windows(2) {
            if complex {
                complex = false;
                continue;
            }
            let l = window[0];
            let r = window[1];
            complex = true;
            ret += match (l as char, r as char) {
                ('I', 'V') => 4,
                ('I', 'X') => 9,
                ('X', 'L') => 40,
                ('X', 'C') => 90,
                ('C', 'D') => 400,
                ('C', 'M') => 900,
                _ => {
                    complex = false;
                    Self::get_value(l)
                }
            };
        }

        if complex {
            ret
        } else {
            ret + Self::get_value(*v.last().unwrap())
        }
    }

    fn get_value(v: u8) -> i32 {
        match v as char {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => {
                unreachable!("{} is not allowed", v as char);
            }
        }
    }
}
