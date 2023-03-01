pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut start_idx = 0;
    let sb = s.as_bytes();
    let sb_len = sb.len();
    for i in 1..sb_len {
        if sb[i] != sb[start_idx] {
            if i - start_idx >= 3 {
                ans.push(vec![start_idx as i32, i as i32 - 1]);
            }
            start_idx = i;
        }
    }
    if sb_len - start_idx >= 3 {
        ans.push(vec![start_idx as i32, sb_len as i32-1]);
    }
    ans
}

mod tests {
    use super::large_group_positions;
    #[test]
    fn test_large_group_positions() {
        let s = String::from("abbxxxxzzy");
        let expected = vec![vec![3,6]];
        let actual = large_group_positions(s);
        assert_eq!(actual, expected);
    }
}
