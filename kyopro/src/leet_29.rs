pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let     pos  = (dividend > 0) == (divisor > 0);
    let mut dd   = ((dividend as i64).abs(), (divisor as i64).abs());
    let mut pow2 = 0;
    let mut quot = 0;

    while dd.1 >> pow2 != 0 { pow2 += 1; }

    for p in (0..=32 - pow2).rev() {
        if dd.1 << p <= dd.0 {
            quot +=    1 << p;
            dd.0 -= dd.1 << p;
        }
    }
    if pos {   quot .min(i32::MAX as i64) as i32 } 
    else   { (-quot).max(i32::MIN as i64) as i32 }
}

mod tests {
    use super::divide;
    #[test]
    fn test_divide() {
        let dividend = 10;
        let divisor = 3;
        let expected = 3;
        let actual = divide(dividend, divisor);
        assert_eq!(actual, expected);
    }
}
