


//Usually when you buy something, you're asked whether your credit card number, phone number or answer to your most secret question is still correct. 
// However, since someone could look over your shoulder, you don't want that shown on your screen. Instead, we mask it.
// Your task is to write a function maskify, which changes all but the last four characters into '#'. 
 

// maskify("4556364607935616") == String::from("############5616");
// maskify("64607935616") == String::from("#######5616");
// maskify("1") == String::from("1");
// maskify("") == String::from("");
// 
// "What was the name of your first pet?"
// maskify("Skippy") == String::from("##ippy");
// maskify("Nananananananananananananananana Batman!") ==String::from("####################################man!");




/// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    match cc.len() {
        0..=4 => cc.to_string(),
        len => {
            cc.chars().take(len-4).map(|_| '#').chain(cc[len-4..].chars()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}