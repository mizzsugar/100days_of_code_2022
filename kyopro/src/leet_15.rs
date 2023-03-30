pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut closest = nums[0] + nums[1] + nums[2];
    let mut min_diff = (closest - target).abs();

    for i in 0..nums.len() - 2 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            let diff = (sum - target).abs();

            if diff < min_diff {
                closest = sum;
                min_diff = diff;
            }

            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else {
                return sum;
            }
        }
    }

    closest
}


mod tests {
    use super::three_sum_closest;
    #[test]
    fn test_three_sum_closest() {
        let nums = vec![-1,2,1,-4];
        let target = 1;
        let expected = 2;
        let actual = three_sum_closest(nums, target);
        assert_eq!(actual, expected);
    }
}
