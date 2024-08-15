package ac

func canJump(nums []int) bool {
	n := len(nums)
	if n <= 1 {
		return true
	}
	for i := 1; i < n; i++ {
		if nums[i-1] < i {
			return false
		}
		nums[i] = max(nums[i]+i, nums[i-1])
	}
	return nums[n-2] >= n-1
}
