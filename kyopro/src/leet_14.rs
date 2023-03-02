pub fn longest_common_prefix(strs: Vec<String>) -> String { 
    match strs.is_empty() {
        true => "".to_string(),
        _ => {
            strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                 acc
                    .chars()
                    .zip(x.chars())
                    .take_while(|(x,y)| x == y)
                    .map(|(x, _)| x)
                    .collect()
            })
        }
    }
}

mod tests {
    use super::longest_common_prefix;
    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
        let expected = String::from("fl");
        let actual = longest_common_prefix(strs);
        assert_eq!(actual, expected);
    }
}
