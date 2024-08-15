package ac

func jumpMemoization(nums []int) int {
	n := len(nums)
	dp := make([]int, n)
	for i := range dp {
		dp[i] = -1
	}

	var solve func(curIdx int) int
	solve = func(curIdx int) int {
		if curIdx >= n-1 {
			return 0
		}
		if dp[curIdx] != -1 {
			return dp[curIdx]
		}

		minJumps := n + 1
		for step := nums[curIdx]; step >= 1; step-- {
			jumps := 1 + solve(curIdx+step)
			minJumps = min(minJumps, jumps)
		}

		dp[curIdx] = minJumps
		return minJumps
	}

	return solve(0)
}

func jumpTabulation(nums []int) int {
	n := len(nums)
	dp := make([]int, n)
	for i := n - 2; i >= 0; i-- {
		minJumps := n + 1
		for step := nums[i]; step >= 1; step-- {
			var jumps int
			if i+step >= n-1 {
				jumps = 1
			} else {
				jumps = 1 + dp[i+step]
			}
			minJumps = min(minJumps, jumps)
		}
		dp[i] = minJumps
	}
	return dp[0]
}

func jumpGreedy(nums []int) int {
	n := len(nums)
	if n <= 1 {
		return 0
	}
	for i := 1; i < n; i++ {
		nums[i] = max(nums[i]+i, nums[i-1])
	}

	curIdx, jumps := 0, 0
	for curIdx < n-1 {
		jumps++
		curIdx = nums[curIdx]
	}

	return jumps
}
