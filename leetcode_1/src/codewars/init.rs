
fn add (a:i32, b:i32) ->  i32 {
    a+b
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(4, add(2,2));
    } 
}
