package ac

func minPathSum(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	for i := m - 2; i >= 0; i-- {
		grid[i][n-1] += grid[i+1][n-1]
	}
	for j := n - 2; j >= 0; j-- {
		grid[m-1][j] += grid[m-1][j+1]
	}

	for i := m - 2; i >= 0; i-- {
		for j := n - 2; j >= 0; j-- {
			grid[i][j] += min(grid[i+1][j], grid[i][j+1])
		}
	}

	return grid[0][0]
}
