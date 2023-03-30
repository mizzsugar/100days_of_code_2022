pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let mut right = 0;
    while right < nums.len() {
        if nums[right] & 1 == 0 {
            nums.swap(left, right);
            left += 1;
        }
        right += 1;
    }
    nums        
}

mod tests {
    use super::sort_array_by_parity;
    #[test]
    fn test_sort_array_by_parity() {
        let nums = vec![3,1,2,4];
        let expected = vec![2,4,3,1];
        let actual = sort_array_by_parity(nums);
        assert_eq!(actual, expected);
    }
}
