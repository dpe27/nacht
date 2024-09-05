package ac

// isInterleave1 func: 2D array
func isInterleave1(s1 string, s2 string, s3 string) bool {
	m, n, l := len(s1), len(s2), len(s3)

	if m+n != l {
		return false
	}

	dp := make([][]bool, m+1)
	for i := range dp {
		dp[i] = make([]bool, n+1)
	}
	dp[0][0] = true

	for i := 1; i <= m; i++ {
		dp[i][0] = dp[i-1][0] && s1[i-1] == s3[i-1]
	}

	for j := 1; j <= n; j++ {
		dp[0][j] = dp[0][j-1] && s2[j-1] == s3[j-1]
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			dp[i][j] = (dp[i-1][j] && s1[i-1] == s3[i+j-1]) || (dp[i][j-1] && s2[j-1] == s3[i+j-1])
		}
	}

	return dp[m][n]
}

func isInterleave(s1 string, s2 string, s3 string) bool {
	m, n, l := len(s1), len(s2), len(s3)
	if m+n != l {
		return false
	}

	if m < n {
		return isInterleave(s2, s1, s3)
	}

	dp := make([]bool, n+1)
	dp[0] = true

	for j := 1; j <= n; j++ {
		dp[j] = dp[j-1] && s2[j-1] == s3[j-1]
	}

	for i := 1; i <= m; i++ {
		dp[0] = dp[0] && s1[i-1] == s3[i-1]
		for j := 1; j <= n; j++ {
			dp[j] = (dp[j] && s1[i-1] == s3[i+j-1]) || (dp[j-1] && s2[j-1] == s3[i+j-1])
		}
	}

	return dp[n]
}
