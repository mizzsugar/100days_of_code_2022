pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
	let mut result = 0;
	let n = grid.len();

	for i in 0..n {
		let m = grid[i].len();

		for j in 0..m {
			if grid[i][j] == 1 {
				result += 4;
				// 上
				if i >= 1 && grid[i - 1][j] == 1 {
					result -= 1;
				}
				// 下
				if i + 1 < n && grid[i + 1][j] == 1 {
					result -= 1;
				}
				// 左
				if j >= 1 && grid[i][j - 1] == 1 {
					result -= 1;
				}
				// 右
				if j + 1 < m && grid[i][j + 1] == 1 {
					result -= 1;
				}
			}
		}
	}

	result
}
mod tests {
    use super::island_perimeter;
    #[test]
    fn test_island_perimeter() {
        let grid = vec![vec![0,1,0,0], vec![1,1,1,0], vec![0,1,0,0], vec![1,1,0,0]];
        let expected = 16;
        let actual = island_perimeter(grid);
        assert_eq!(actual, expected);
    }
}
