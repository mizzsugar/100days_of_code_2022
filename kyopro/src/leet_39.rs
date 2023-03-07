pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn dfs(candidates:&Vec<i32>, target:i32, i:usize, cur: &mut Vec<i32>, res:&mut Vec<Vec<i32>>){
        if target == 0 {
            res.push(cur.clone());
            return;
        }  else if i>=candidates.len() || target < 0{
            return;
        }
        let c = candidates[i];
        cur.push(c);
        dfs(candidates, target - c, i, cur, res);
        cur.pop();
        dfs(candidates, target, i + 1, cur, res);
    }
    
    let mut res = vec![];
    dfs(&candidates, target, 0, &mut vec![], &mut res);
    res
}

mod tests {
    use super::combination_sum;
    #[test]
    fn test_combination_sum() {
        let candidates = vec![2,3,6,7];
        let target = 7;
        let expected =  vec![vec![2,2,3], vec![7]];
        let actual = combination_sum(candidates, target);
        assert_eq!(actual, expected);
    }
}
