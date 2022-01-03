/// Converts a number to a string representating roman numeral.
fn num_as_roman(mut num: i32) -> String {
    let mut letters = String::new();
    let symbols = [(1000, "M"), (900, "CM"),
                   (500,  "D"), (400, "CD"), 
                   (100,  "C"), (90,  "XC"),
                   (50,   "L"), (40,  "XL"), 
                   (10,   "X"), (9,   "IX"),
                   (5,    "V"), (4,   "IV"), 
                   (1,    "I")];
     for &(n, symbol) in symbols.iter() {
       while num >= n {
           letters.push_str(symbol);
           num -= n;
       }
    }
    letters
  }

#[test]
fn returns_expected() {
    assert_eq!(num_as_roman(182), "CLXXXII");
    assert_eq!(num_as_roman(1990), "MCMXC");
    assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
