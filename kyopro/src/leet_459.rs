pub fn repeated_substring_pattern(s: String) -> bool {
    let s = s
        .chars()
        .collect::<Vec<_>>();
    (1..s.len())
    .filter(|i| s.len() % i == 0)
    .any(|i| s.chunks(i).all(|chunk| *chunk == s[..i]))
}

mod tests {
    use super::repeated_substring_pattern;
    #[test]
    fn test_repeated_substring_pattern() {
        let s = String::from("abab");
        let expected = true;
        let actual = repeated_substring_pattern(s);
        assert_eq!(actual, expected);
    }
}
