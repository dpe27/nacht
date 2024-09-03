package ac

func minDistance(word1 string, word2 string) int {
	m, n := len(word1), len(word2)
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
		for j := range dp[i] {
			dp[i][j] = -1
		}
	}

	var solve func(s1, s2 string, i, j int) int
	solve = func(s1, s2 string, i, j int) int {
		if i == 0 {
			return j
		}
		if j == 0 {
			return i
		}
		if dp[i][j] != -1 {
			return dp[i][j]
		}
		if s1[i-1] == s2[j-1] {
			dp[i][j] = solve(s1, s2, i-1, j-1)
			return dp[i][j]
		}

		ins := 1 + solve(s1, s2, i, j-1)
		del := 1 + solve(s1, s2, i-1, j)
		rep := 1 + solve(s1, s2, i-1, j-1)
		dp[i][j] = min(ins, del, rep)
		return dp[i][j]
	}

	return solve(word1, word2, m, n)
}
