pub fn is_valid(s: String) -> bool {
    let mut m = Vec::new();

    for chr in s.chars() {
        match chr {
            '(' => m.push('p') ,
            ')' => match m.pop() {
                Some(c) => if c != 'p' {return false;},
                None => return false,
            }, 
            '{' => m.push('s'), 
            '}' => match m.pop() {
                Some(c) => if c != 's' {return false;},
                None => return false,
            },
            '[' => m.push('q'), 
            ']' => match m.pop() {
                Some(c) => if c != 'q' {return false;},
                None => return false,
            }, 
            _   => println!("all"),
        } 
    }
    
    m.is_empty()
}

mod tests {
    use super::is_valid;
    #[test]
    fn test_is_valid() {
        let s = String::from("()");
        let expected = true;
        let actual = is_valid(s);
        assert_eq!(actual, expected);
    }
}
