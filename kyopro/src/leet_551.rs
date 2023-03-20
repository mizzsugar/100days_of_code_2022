pub fn check_record(s: String) -> bool {
    if s.contains("LLL"){ return false}
    return if s.chars().filter(|c| c == &'A').count() > 1 {false } else {true}
}

mod tests {
    use super::check_record;
    #[test]
    fn test_check_record() {
        let s = String::from("PPALLP");
        let expected = true;
        let actual = check_record(s);
        assert_eq!(actual, expected);
    }
}
