package ac

func getRow(rowIndex int) []int {
	prev := []int{}
	for i := 0; i <= rowIndex; i++ {
		curr := make([]int, i+1)
		for idx := range curr {
			curr[idx] = 1
		}

		for j := 1; j < i; j++ {
			curr[j] = prev[j-1] + prev[j]
		}

		prev = curr
	}

	return prev
}
