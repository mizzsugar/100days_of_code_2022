pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let len = candy_type.len();
    let hashset: HashSet<i32> = candy_type.into_iter().collect();
    std::cmp::min(len / 2, hashset.len()) as i32
}

mod tests {
    use super::distribute_candies;
    #[test]
    fn test_distribute_candies() {
        let candy_type = vec![1,1,2,2,3,3];
        let expected = 3;
        let actual = distribute_candies(candy_type);
        assert_eq!(actual, expected);
    }
}
