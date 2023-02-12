pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max: i32 = 0;
    for i in &accounts {
        let mut wealth: i32 = 0;
        for j in i{
            wealth+=j;
        }   
        if max<wealth{
            max = wealth;
        }
    }   
    max
}

#[cfg(test)]
mod tests {
    use super::maximum_wealth;
    #[test]
    fn test_maximum_wealth() {
        let accounts = vec![vec![1,2,3], vec![3,2,1]];
        let mut actual = maximum_wealth(accounts);
        let expected = 6;

        assert_eq!(actual, expected);
    }
}
