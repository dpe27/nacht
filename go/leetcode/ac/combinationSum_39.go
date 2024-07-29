package ac

func combinationSum(candidates []int, target int) [][]int {
	var res [][]int
	var backtracking func(canIdx int, combination []int, total int)

	backtracking = func(canIdx int, combination []int, total int) {
		if total == target {
			tmp := make([]int, len(combination))
			copy(tmp, combination)
			res = append(res, tmp)
			return
		}

		if canIdx >= len(candidates) || total > target {
			return
		}

		combination = append(combination, candidates[canIdx])
		backtracking(canIdx, combination, total+candidates[canIdx])
		combination = combination[:len(combination)-1]
		backtracking(canIdx+1, combination, total)
	}

	backtracking(0, []int{}, 0)
	return res
}
