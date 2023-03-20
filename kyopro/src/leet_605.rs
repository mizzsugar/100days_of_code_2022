pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut f = flowerbed;
    let mut n = n;
    if n == 0 {
        return true;
    }
    if n == 1 && f.len() == 1 && f[0] == 0 {
        return true;
    }
    for i in 0..f.len() {
        if i == 0 && i< f.len()-1 && f[i] == 0 && f[i+1] == 0 {
            f[i] = 1;
            n -= 1;
        } else if i > 0 && i == f.len()-1 && f[i-1] == 0 && f[i] == 0 {
            f[i] = 1;
            n -= 1;
        } else if i >0 && i < f.len()-1 && f[i-1] == 0 && f[i] == 0 && f[i+1] == 0 {
             f[i] = 1;
             n -= 1;
        }
        if n == 0 {
            return true;
        }
    }
    return n == 0;
}

mod tests {
    use super::can_place_flowers;
    #[test]
    fn test_can_place_flowers() {
        let flowerbed = vec![1,0,0,0,1];
        let expected = true;
        let actual = can_place_flowers(flowerbed);
        assert_eq!(actual, expected);
    }
}
