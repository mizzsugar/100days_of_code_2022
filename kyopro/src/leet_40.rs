pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn dfs(candidates: &Vec<i32>, target: i32, start:usize, cur: &mut Vec<i32>,  res: &mut Vec<Vec<i32>>){
        if target == 0 {
            res.push(cur.clone());
            return;
        } else if target < 0 {
            return;
        }
        for i in start..candidates.len(){
            if i > start && candidates[i] == candidates[i-1]{
                continue;
            }
            let n = candidates[i];
            cur.push(n);
            dfs(candidates, target-n, i+1, cur, res);
            cur.pop();
        }
    }
    candidates.sort();
    let mut res = vec![];
    dfs(&candidates, target, 0, &mut vec![], &mut res);
    res
}

mod tests {
    use super::combination_sum2;
    #[test]
    fn test_combination_sum2() {
        let candidates = vec![10,1,2,7,6,1,5];
        let target = 8;
       
        let expected =  vec![
            vec![1,1,6],
            vec![1,2,5],
            vec![1,7],
            vec![2,6]
        ];
        let actual = combination_sum2(candidates, target);
        assert_eq!(actual, expected);
    }
}
