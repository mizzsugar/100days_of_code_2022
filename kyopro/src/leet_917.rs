pub fn reverse_only_letters(mut s: String) -> String {
    let bytes = unsafe { s.as_bytes_mut() };
    let (mut lo, mut hi) = (0, bytes.len() - 1);

    while lo < hi {
        while lo < hi && !bytes[lo].is_ascii_alphabetic() {
            lo += 1;
        }
        while lo < hi && !bytes[hi].is_ascii_alphabetic() {
            hi -= 1;
        }
        bytes.swap(lo, hi);
        lo += 1;
        hi -= 1;
    }
    s
}

mod tests {
    use super::reverse_only_letters;
    #[test]
    fn test_reverse_only_letters() {
        let s = String::from("ab-cd");
        let expected = String::from("dc-ba");;
        let actual = reverse_only_letters(s);
        assert_eq!(actual, expected);
    }
}
