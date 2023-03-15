pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut lookup: HashSet<i32> = HashSet::new();

    let k = k as usize;
    
    for i in 0..nums.len() {
        if i > k {
            lookup.remove(&nums[i-k-1]);
        }
        if !lookup.insert(nums[i]) { 
            return true;
        }
    }

    false
}

mod tests {
    use super::contains_nearby_duplicate;
    #[test]
    fn test_contains_nearby_duplicate() {
        let nums = vec![1,2,3,1];
        let k = 3;
        let expected = true;
        let actual = contains_nearby_duplicate(nums, k);
        assert_eq!(actual, expected);
    }
}
