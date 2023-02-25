pub fn get_hint(secret: String, guess: String) -> String {
    let mut contains_secret = vec![0;10];
    let mut contains_guess = vec![0;10];
    let mut a = 0;
    for (s, g) in secret.chars().zip(guess.chars()) {
        if s == g {
            a += 1;
            continue
        }
        contains_secret[s.to_digit(10).unwrap() as usize] += 1;
        contains_guess[g.to_digit(10).unwrap() as usize] += 1;
    }
    let b = contains_secret.iter().zip(contains_guess.iter()).fold(0, |mut init, (&s, &g)| {
        init += s.min(g);
        init
    }
    );
    format!("{a}A{b}B")
}

mod tests {
    use super::get_hint;
    #[test]
    fn test_get_hint() {
        let secret = String::from("1807");
        let guess = String::from("7810");
        let expected = String::from("1A1B");
        let actual = get_hint(secret, guess);
        assert_eq!(actual, expected);
    }
}
