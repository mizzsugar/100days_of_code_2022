pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }
    let mut ans = vec![];
    let mut interval = (nums[0], nums[0]);
    for i in 1..nums.len() {
        if nums[i] - nums[i - 1] == 1 {
            interval.1 = nums[i];
        } else {
            ans.push(interval);
            interval = (nums[i], nums[i]);
        }
    }
    ans.push(interval);
    ans.into_iter()
        .map(|interval| {
            if interval.0 == interval.1 {
                format!("{}", interval.0)
            } else {
                format!("{}->{}", interval.0, interval.1)
            }
        })
        .collect()  
}

mod tests {
    use super::summary_ranges;
    #[test]
    fn summary_ranges() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let expected = vec![String::from("0->2"), String::from("4->5"), String::from("7")];
        let mut actual = summary_ranges(nums);
        assert_eq!(actual, expected);
    }

}
