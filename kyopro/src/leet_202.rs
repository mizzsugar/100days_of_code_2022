pub fn is_happy(mut n: i32) -> bool {
    while n > 9 {
        let mut tmp = 0;
        while n > 0 {
            tmp += (n % 10).pow(2);
            n /= 10;
        }
        n = tmp;
    }
    if n == 1 || n == 7 { true } else { false }
}

mod tests {
    use super::is_happy;
    #[test]
    fn test_is_happy() {
        let num = 19;
        let expected = true;
        let actual = is_happy(num);
        assert_eq!(actual, expected);
    }
}
