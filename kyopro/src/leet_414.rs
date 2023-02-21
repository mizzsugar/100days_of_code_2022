pub fn third_max(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable_by(|a, b| b.cmp(&a));
    nums.dedup();
    nums[if nums.len() > 2 { 2 } else { 0 }]
}

mod tests {
    use super::third_max;
    #[test]
    fn test_third_max() {
        let nums = vec![3, 2, 1];
        let expected = 1;
        let actual = third_max(num);
        assert_eq!(actual, expected);
    }
}
