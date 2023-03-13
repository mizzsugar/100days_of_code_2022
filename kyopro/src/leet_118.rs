pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut triangle = Vec::with_capacity(num_rows as usize);
    triangle.push(vec![1]);
    for i in 1..num_rows {
        let mut row = Vec::with_capacity(i as usize);
        row.push(1);
        let prev_row = &triangle[(i - 1) as usize];

        for j in 1..prev_row.len() {
            let val = prev_row[j - 1] + prev_row[j];
            row.push(val);
        }
        row.push(1);
        triangle.push(row);
    }
    triangle
}

mod tests {
    use super::generate;
    #[test]
    fn test_generate() {
        let num_rows = 5;
        let expected = vec![
            vec![1],
            vec![1,1],
            vec![1,2,1],
            vec![1,3,3,1],
            vec![1,4,6,4,1],
        ];
        let actual = generate(num_rows);
        assert_eq!(actual, expected);
    }
}
