package ac

func combine(n int, k int) [][]int {
	var res [][]int
	var backtrack func([]int, int)

	backtrack = func(combination []int, idx int) {
		if len(combination) == k {
			tmp := make([]int, len(combination))
			copy(tmp, combination)
			res = append(res, tmp)
		}
		for i := idx; i <= n; i++ {
			combination = append(combination, i)
			backtrack(combination, i+1)
			combination = combination[:len(combination)-1]
		}
	}
	backtrack([]int{}, 1)
	return res
}
