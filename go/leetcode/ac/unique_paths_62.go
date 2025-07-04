package ac

func uniquePaths(m int, n int) int {
	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, n)
	}

	dp[m-1][n-1] = 1

	for j := n - 2; j >= 0; j-- {
		dp[m-1][j] = 1
	}

	for i := m - 2; i >= 0; i-- {
		dp[i][n-1] = 1
	}

	for i := m - 2; i >= 0; i-- {
		for j := n - 2; j >= 0; j-- {
			dp[i][j] = dp[i+1][j] + dp[i][j+1]
		}
	}
	return dp[0][0]
}
