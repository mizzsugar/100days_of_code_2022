pub fn is_power_of_two(mut n: i32) -> bool {
    match n {
        i32::MIN..=0 => false,
        1 => true,
        _ => {
            while n > 1 {
                if n % 2 != 0 { return false; }
                n /= 2;
            }
            true
        }
    }
    
}


mod tests {
    use super::is_power_of_two;
    #[test]
    fn test_is_power_of_two() {
        let n = 16;
        let expected = true;
        let actual = is_power_of_two(n);
        assert_eq!(actual, expected);
    }
}
