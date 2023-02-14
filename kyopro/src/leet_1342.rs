pub fn number_of_steps(num: i32) -> i32 {
    match num > 1 {
        true => (num.count_zeros() - num.leading_zeros() + 2 * num.count_ones() - 1) as i32,
        false => num,
    }
}

#[cfg(test)]
mod tests {
    use super::number_of_steps;
    #[test]
    fn test_number_of_steps() {
        let num = 14;
        let mut actual = number_of_steps(num);
        let expected = 6;

        assert_eq!(actual, expected);
    }
}
