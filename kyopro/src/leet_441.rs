pub fn arrange_coins(n: i32) -> i32 {
    let x = ((0.25 + 2.0 * (n as f64)).sqrt() - 0.5).floor() as i32;
    return x;
}

mod tests {
    use super::arrange_coins;
    #[test]
    fn test_arrange_coins() {
        let num = 6;
        let expected = 2;
        let actual = arrange_coins(num);
        assert_eq!(actual, expected);
    }
}
