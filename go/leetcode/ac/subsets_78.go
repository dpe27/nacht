package ac

// subsets (Approach: Backtracking)
func subsets(nums []int) [][]int {
	var res [][]int
	var backtrack func([]int, int)
	backtrack = func(subset []int, start int) {
		res = append(res, append([]int{}, subset...))
		for i := start; i < len(nums); i++ {
			subset = append(subset, nums[i])
			backtrack(subset, i+1)
			subset = subset[:len(subset)-1]
		}
	}
	backtrack([]int{}, 0)
	return res
}

// subsetBit (Approach: Bit Manipulation)
func subsetBit(nums []int) [][]int {
	var res [][]int
	for i := 0; i < (1 << len(nums)); i++ {
		var subset []int
		for j := 0; j < len(nums); j++ {
			if i&(1<<j) > 0 {
				subset = append(subset, nums[j])
			}
		}
		res = append(res, subset)
	}
	return res
}
