use std::collections::{HashSet, HashMap};

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut word = String::new();
    let (mut common, mut max_count) = ("".to_string(), 0);

    let mut banned_set: HashSet<String> = HashSet::new();

    for word in banned {
        banned_set.insert(word);
    }

    for (index, x) in paragraph.chars().enumerate() {
        if x.is_alphabetic() {
            word = format!("{}{}", word, x.to_lowercase());
            if index != paragraph.len() - 1 {
                continue;
            }
        };

        if word != "" && !banned_set.contains(&word) {
            *counts.entry(word.to_string()).or_insert(1) += 1;
            if counts[&word] > max_count {
                common = word.to_string();
                max_count = counts[&word];
            }
        }
        word = String::new();
    }
    common
}



mod tests {
    use super::most_common_word;
    #[test]
    fn test_most_common_word() {
        let paragraph = String::from("Bob hit a ball, the hit BALL flew far after it was hit.");
        let banned = vec![String::from("hit")];
        let expected = String::from("ball");
        let actual = most_common_word(paragraph, banned);
        assert_eq!(actual, expected);
    }
}
