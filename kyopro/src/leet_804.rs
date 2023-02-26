pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let mut s:HashSet<String> = HashSet::new();
    let table = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
    for ss in words {
        let mut key = String::new();
        for b in ss.bytes(){
            key.push_str(table[(b as usize)-('a' as usize)]);
        }
        s.insert(key);
    }
    s.len() as i32
}

mod tests {
    use super::unique_morse_representations;
    #[test]
    fn test_unique_morse_representations() {
        let words = vec![String::from("gin"), String::from("zen"), String::from("gig"), String::from("msg")];
        let expected = 2;
        let actual = unique_morse_representations(words);
        assert_eq!(actual, expected);
    }
}
