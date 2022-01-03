// create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"

fn create_phone_number(x: &[u8]) -> String {
    format!("({}{}{}) {}{}{}-{}{}{}{}", x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], x[8], x[9])
}

#[test]
fn returns_expected() {
  assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
  assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
  assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
}