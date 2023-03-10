pub fn add_binary(a: String, b: String) -> String {
    let bin_to_ch = ['0', '1', '0', '1'];
    let mut it_a = a.chars().rev();
    let mut it_b = b.chars().rev();
    let mut of = 0;
    let mut res = String::new();
    loop {
        let (bit_a, bit_b) = match (it_a.next(), it_b.next()) {
            (Some(bit_a), Some(bit_b)) => 
                (bit_a as u8 - 48, bit_b as u8 - 48),
            (Some(bit_x), None) | (None, Some(bit_x)) =>
                (bit_x as u8 - 48, 0),
            (None, None) => break,
        };
        res.push(bin_to_ch[(bit_a + bit_b + of) as usize]);
        if bit_a + bit_b + of < 2 {
            of = 0;
        } else {
            of = 1;
        }
    }
    if of == 1 {
        res.push('1');
    }
    res.chars().rev().collect::<String>()
}

mod tests {
    use super::add_binary;
    #[test]
    fn test_add_binary() {
        let a = String::from("11");
        let b = String::from("1");
       
        let expected = String::from("100");
        let actual = add_binary(a, b);
        assert_eq!(actual, expected);
    }
}
