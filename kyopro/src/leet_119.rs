pub fn get_row(row_index: i32) -> Vec<i32> {
    let n = row_index as usize;
    let mut row = vec![0; n + 1];

    for i in 0..n + 1 {
        for j in (0..i + 1).rev().step_by(1) {
            if j == 0 || j == i {
                row[j] = 1;
            } else {
                row[j] += row[j - 1];
            }
        }
    }

    row
}

mod tests {
    use super::get_row;
    #[test]
    fn test_get_row() {
        let row_index = 3;
        let expected = vec![1,3,3,1];
        let actual = get_row(row_index);
        assert_eq!(actual, expected);
    }
}
