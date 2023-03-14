pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for x in &nums {
        set.insert(x);
    }
    set.len() != nums.len()
}

mod tests {
    use super::contains_duplicate;
    #[test]
    fn test_contains_duplicate() {
        let nums = vec![1,2,3,1];
        let expected = true;
        let actual = contains_duplicate(nums);
        assert_eq!(actual, expected);
    }
}
