pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let nu = turned_on as u32;
    let mut result = Vec::new();
    for i in 0i32..12 {
        for j in 0i32..60 {
            if (i.count_ones() + j.count_ones()) == nu {
                result.push(format!("{}:{:02}", i, j));
            }
        }
    }
    result
}

mod tests {
    use super::read_binary_watch;
    #[test]
    fn test_read_binary_watch() {
        let turned_on = 1
        let expected = vec![
            String::from("0:01"),
            String::from("0:02"),
            String::from("0:04"),
            String::from("0:08"),
            String::from("0:16"),
            String::from("0:32"),
            String::from("1:00"),
            String::from("2:00"),
            String::from("4:00"),
            String::from("8:00"),
        ];
        let actual = read_binary_watch(turned_on);
        assert_eq!(actual, expected);
    }
}
