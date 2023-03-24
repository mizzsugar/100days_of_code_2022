pub fn backspace_compare(s: String, t: String) -> bool {
    fn get_str(s: String) -> Vec<char> {
        let mut stack = Vec::new();
        s.chars().for_each(|c| match c {
            '#' => {
                stack.pop();
            }
            _ => stack.push(c),
        });
        stack
    }
    get_str(s) == get_str(t)
}


mod tests {
    use super::backspace_compare;
    #[test]
    fn test_backspace_compare() {
        let s = String::from("ab#c");
        let t = String::from("ad#c");
        let expected = true;
        let actual = backspace_compare(s, t);
        assert_eq!(actual, expected);
    }
}
