pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut combinations: Vec<Vec<String>> = vec![vec!["".to_string()], vec!["()".to_string()]];
    for k in 2..=n as usize {
        let mut k_combination = vec![];
        for c in 0..k {
            for left in &combinations[c] {
                for right in &combinations[k - 1 - c] {
                    k_combination.push(format!("({}){}", left, right))
                }
            }
        }
        combinations.push(k_combination)
    }
    combinations.pop().unwrap()
}

mod tests {
    use super::generate_parenthesis;
    #[test]
    fn test_generate_parenthesis() {
        let n = 3;
        let expected = vec![
            String::from("((()))"),
            String::from("(()())"),
            String::from("(())()"),
            String::from("()(())"),
            String::from("()()()"),
        ];
        let actual = generate_parenthesis(n);
        assert_eq!(actual, expected);
    }
}
