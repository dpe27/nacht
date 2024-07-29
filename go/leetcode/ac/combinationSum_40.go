package ac

import "sort"

func combinationSum2(candidates []int, target int) [][]int {
	n := len(candidates)
	var res [][]int
	sort.Ints(candidates)
	var backtrack func([]int, int, int)
	backtrack = func(combination []int, total, idx int) {
		if total == target {
			tmp := append([]int{}, combination...)
			res = append(res, tmp)
			return
		}
		if total > target || idx == n {
			return
		}
		for i := idx; i < n; i++ {
			if i != idx && candidates[i] == candidates[i-1] {
				continue
			}
			combination = append(combination, candidates[i])
			backtrack(combination, total+candidates[i], i+1)
			combination = combination[:len(combination)-1]
		}
	}
	backtrack([]int{}, 0, 0)
	return res
}
