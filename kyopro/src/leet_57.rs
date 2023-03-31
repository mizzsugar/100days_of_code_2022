pub fn insert(vals: Vec<Vec<i32>>, mut val: Vec<i32>) -> Vec<Vec<i32>> {
    let mut vec = Vec::new();

    let n = vals.len();
    let mut i = 0;

    while i < n && vals[i][1] < val[0] {
        vec.push(vals[i].clone());
        i += 1;
    }

    while i < n && (vals[i][0] <= val[1] && val[0] <= vals[i][1]) {
        val[0] = val[0].min(vals[i][0]);
        val[1] = val[1].max(vals[i][1]);
        i += 1;
    }

    vec.push(val);

    while i < n {
        vec.push(vals[i].clone());
        i += 1;
    }

    vec
}

mod tests {
    use super::insert;
    #[test]
    fn test_insert() {
        let intervals = vec![vec![1,3], vec![6,9]];
        let new_interval = vec![2,5];
        let expected = vec![vec![1,5],vec![6,9]];
        let actual = insert(intervals, new_interval);
        assert_eq!(actual, expected);
    }
}
