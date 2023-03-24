pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut mon_dec = true;
    let mut mon_inc = true;
    
    for i in 1..nums.len() {
        if nums[i-1] > nums[i] {
            mon_dec = false;
        }
        if nums[i-1] < nums[i] {
            mon_inc = false;
        }
    }        
    return mon_dec || mon_inc;
}

mod tests {
    use super::is_monotonic;
    #[test]
    fn test_is_monotonic() {
        let nums = vec![1,2,2,3];
        let expected = true;
        let actual = is_monotonic(nums);
        assert_eq!(actual, expected);
    }
}
