pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut perms = Vec::new();
    let mut temp  = Vec::new();
    perm(&mut perms, &mut temp, &nums);
    perms
}

pub fn perm(perms: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>) {
    if temp.len() == nums.len() {
        perms.push(temp.to_vec());
        return;
    }
    
    for n in 0..nums.len() {
        if !temp.contains(&nums[n]) {
            temp.push(nums[n]);
            perm(perms, temp, nums);
            temp.pop();   
        }
    }
}

mod tests {
    use super::permute;
    #[test]
    fn test_permute() {
        let nums = vec![1,2,3];
       
        let expected =  vec![
            vec![1,2,3],
            vec![1,3,2],
            vec![2,1,3],
            vec![2,3,1],
            vec![3,1,2],
            vec![3,2,1],
        ];
        let actual = permute(nums);
        assert_eq!(actual, expected);
    }
}
