pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut res = vec![0;0];

    for i in operations {
        match i.parse::<i32>() {
            Ok(number) => {
                res.push(number);
            },
            Err(e) => {
                let n = res.len();
                match i.as_str() {
                    "+" => {
                        res.push(
                            res[n-1] + res[n-2]
                        )
                    },
                    "D" => {
                        res.push(res[n-1] * 2);
                    },
                    "C" => {
                        res.pop();
                    },
                    _ => {}
                }
            },
        }
    }

    res.iter().sum()
}


mod tests {
    use super::cal_points;
    #[test]
    fn test_cal_points() {
        let ops = vec![String::from("5"), String::from("2"), String::from("C"), String::from("D"), String::from("+")];
        let expected = 30;
        let actual = cal_points(ops);
        assert_eq!(actual, expected);
    }
}
