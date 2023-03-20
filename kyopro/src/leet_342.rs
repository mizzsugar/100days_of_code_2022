pub fn is_power_of_four(n: i32) -> bool {
    if n < 1 {
        return false;
    }
    let mut x = 0;
    let mut pos = 0;
    while pos < 32 {
        if x > 1 {
            return false;
        }
        let mut t = 1;
        t = t<<pos;
        if (n & t > 0) {
            if pos%2 != 0 {
                return false;
            }
            x += 1;
        }
        pos += 1;
    }
    true
}

mod tests {
    use super::is_power_of_four;
    #[test]
    fn test_is_power_of_four() {
        let n = 16;
        let expected = true;
        let actual = is_power_of_four(n);
        assert_eq!(actual, expected);
    }
}
