pub fn first_missing_positive( mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        while nums[i] > 0 && nums[i] <= nums.len() as i32 && (nums[i]-1) != (i as i32) && (nums[i] != nums[(nums[i]-1) as usize]) {
            let mut ans = nums[i]-1;
            nums.swap(i,ans as usize);
        }
    }

    for i in 0..nums.len() {
        if nums[i] != (i + 1) as i32 {
            return (i+1) as i32
        }
    }

    (nums.len() + 1) as i32
}

mod tests {
    use super::first_missing_positive;
    #[test]
    fn test_first_missing_positive() {
        let nums = vec![1,2,0]
        let expected = 3;
        let actual = first_missing_positive(nums);
        assert_eq!(actual, expected);
    }
}
