package ac

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	m, n := len(obstacleGrid), len(obstacleGrid[0])
	if obstacleGrid[0][0] == 1 || obstacleGrid[m-1][n-1] == 1 {
		return 0
	}

	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, n)
	}

	dp[m-1][n-1] = 1
	for i := m - 2; i >= 0; i-- {
		if obstacleGrid[i][n-1] == 1 {
			break
		}
		dp[i][n-1] = 1
	}
	for j := n - 2; j >= 0; j-- {
		if obstacleGrid[m-1][j] == 1 {
			break
		}
		dp[m-1][j] = 1
	}

	for i := m - 2; i >= 0; i-- {
		for j := n - 2; j >= 0; j-- {
			if obstacleGrid[i][j] != 1 {
				dp[i][j] = dp[i+1][j] + dp[i][j+1]
			}
		}
	}

	return dp[0][0]
}
