package ac

func permuteUnique(nums []int) [][]int {
	freq := make(map[int]int)
	var res [][]int
	var backtracking func(freq map[int]int, perm []int)

	for _, n := range nums {
		freq[n]++
	}

	backtracking = func(freq map[int]int, perm []int) {
		if len(perm) == len(nums) {
			tmp := make([]int, len(perm))
			copy(tmp, perm)
			res = append(res, tmp)
			return
		}

		for k, v := range freq {
			if v > 0 {
				perm = append(perm, k)
				freq[k]--
				backtracking(freq, perm)
				perm = perm[:len(perm)-1]
				freq[k]++
			}
		}
	}

	backtracking(freq, []int{})
	return res
}
