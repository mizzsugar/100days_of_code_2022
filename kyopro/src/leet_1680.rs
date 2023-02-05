pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum_vector: Vec<i32> = nums.clone();
    for i in 1..sum_vector.len() {
        sum_vector[i] += sum_vector[i - 1];
    }
    sum_vector
}

#[cfg(test)]
mod tests {
    use super::running_sum;
    #[test]
    fn test_running_sum() {
        let nums = vec![3, 1, 2, 10, 1];
        let mut actual = running_sum(nums);
        let expected = vec![3, 4, 6, 16, 17];

        assert_eq!(actual, expected);
    }
}
