pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut sorted_nums = nums.clone();

    sorted_nums.sort_unstable();

    let mut remaining_k = k;

    let mut negated_nums: Vec<i32> = sorted_nums
        .iter()
        .map(|&num| {
            if num < 0 && remaining_k > 0 {
                remaining_k -= 1;
                -num
            } else {
                num
            }
        })
        .collect();

        if remaining_k % 2 != 0 {
        let min_idx = negated_nums
            .iter()
            .enumerate()
            .min_by_key(|(_, &x)| x)
            .unwrap()
            .0;
        negated_nums[min_idx] = -negated_nums[min_idx];
    }

    negated_nums.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::largest_sum_after_k_negations;
    #[test]
    fn test_largest_sum_after_k_negations() {
        let nums = vec[4,2,3];
        let k = 1;
        let mut actual = largest_sum_after_k_negations(nums, k);
        let expected = 3;

        assert_eq!(actual, expected);
    }
}
