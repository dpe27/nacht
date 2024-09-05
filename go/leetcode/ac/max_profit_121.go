package ac

func maxProfit(prices []int) int {
	n := len(prices)
	maxPrice := prices[n-1]
	profit := make([]int, n)

	for i := n - 2; i >= 0; i-- {
		if maxPrice-prices[i] > 0 {
			profit[i] = max(maxPrice-prices[i], profit[i+1])
		} else {
			profit[i] = profit[i+1]
		}

		maxPrice = max(maxPrice, prices[i])
	}

	return profit[0]
}
