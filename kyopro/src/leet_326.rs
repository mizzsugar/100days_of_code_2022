pub fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let mut n = n;
    while n % 3 == 0 {
        n /= 3;
    }
    n == 1
}


mod tests {
    use super::is_power_of_three;
    #[test]
    fn test_is_power_of_three() {
        let n = 27;
        let expected = true;
        let actual = is_power_of_three(n);
        assert_eq!(actual, expected);
    }
}
