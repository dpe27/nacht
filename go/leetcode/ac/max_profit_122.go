package ac

func maxProfitDP(prices []int) int {
	n := len(prices)
	dp := make([]int, n)
	for i := n - 2; i >= 0; i-- {
		dp[i] = dp[i+1]
		if diff := prices[i+1] - prices[i]; diff > 0 {
			dp[i] += diff
		}
	}
	return dp[0]
}
