pub fn count_bits(n: i32) -> Vec<i32> {
    let mut ans = vec![];

    for i in 0..n+1 {
        let (mut cnt, mut n_of_bits) = (i, 0);

        while cnt > 0 {
            n_of_bits+= 1;
            cnt&= cnt-1;
        }
        ans.push(n_of_bits)
    } 
    ans
 }


mod tests {
    use super::count_bits;
    #[test]
    fn test_count_bits() {
        let n = 2;
        let expected = vec![0,1,1];
        let actual = count_bits(n);
        assert_eq!(actual, expected);
    }
}
