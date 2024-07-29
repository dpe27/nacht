package ac

func permute(nums []int) [][]int {
	var res [][]int
	var backtracking func(nums []int, subarray []int)

	backtracking = func(nums []int, subarray []int) {
		if len(nums) == 0 {
			res = append(res, subarray)
			return
		}

		for i := 0; i < len(nums); i++ {
			numsTmp := make([]int, len(nums))
			copy(numsTmp, nums)
			subarrayTmp := make([]int, len(subarray))
			copy(subarrayTmp, subarray)

			backtracking(append(numsTmp[:i], numsTmp[i+1:]...), append(subarrayTmp, nums[i]))
		}
	}

	backtracking(nums, []int{})
	return res
}
