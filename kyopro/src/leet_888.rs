pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let delta:i32 = (alice_sizes.iter().sum::<i32>() - bob_sizes.iter().sum::<i32>())/2;
    let mut s:HashSet<i32> = alice_sizes.into_iter().collect();
    for b in bob_sizes {
        if s.contains(&(b+delta)) {
            return vec![b+delta, b];
        }
    }
    vec![]
}


mod tests {
    use super::fair_candy_swap;
    #[test]
    fn test_backspace_compare() {
        let aliceSizes = vec![1,1];
        let bobSizes = vec![2,2];
        let expected = vec![1,2];
        let actual = fair_candy_swap(aliceSizes, bobSizes);
        assert_eq!(actual, expected);
    }
}
