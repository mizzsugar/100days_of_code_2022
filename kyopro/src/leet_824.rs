pub fn to_goat_latin(sentence: String) -> String {
    let vowels = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

    sentence
        .split_whitespace()
        .enumerate()
        .map(|(idx, word)| {
            let word = match word.starts_with(&vowels) {
                true => word.to_string(),
                false => format!("{}{}", &word[1..], &word[0..1]),
            };

            format!("{word}ma{}", &"a".repeat(idx + 1))
        })
        .collect::<Vec<String>>()
        .join(" ")
}

mod tests {
    use super::to_goat_latin;
    #[test]
    fn test_to_goat_latin() {
        let sentence = String::from("I speak Goat Latin");
        let expected = String::from("Imaa peaksmaaa oatGmaaaa atinLmaaaaa");
        let actual = to_goat_latin(sentence);
        assert_eq!(actual, expected);
    }
}
