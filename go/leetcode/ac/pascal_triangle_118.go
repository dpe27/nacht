package ac

func generate(numRows int) [][]int {
	res := [][]int{}
	prevRow := []int{}

	for i := 0; i < numRows; i++ {
		curr := make([]int, i+1)
		for idx := range curr {
			curr[idx] = 1
		}
		for j := 1; j < i; j++ {
			curr[j] = prevRow[j-1] + prevRow[j]
		}

		res = append(res, curr)
		prevRow = curr
	}
	return res
}
