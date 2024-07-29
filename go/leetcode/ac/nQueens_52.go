package ac

func TotalNQueens(n int) int {
	leftDia := make([]bool, 2*n)
	for i := range leftDia {
		leftDia[i] = false
	}

	rightDia := make([]bool, 2*n-1)
	for i := range rightDia {
		rightDia[i] = false
	}

	row := make([]bool, n)
	for i := range row {
		row[i] = false
	}

	var res int
	var backtrack func(col int)

	backtrack = func(col int) {
		if col >= n {
			res++
			return
		}

		for i := 0; i < n; i++ {
			if !leftDia[i-col+n] && !rightDia[i+col] && !row[i] {
				leftDia[i-col+n] = true
				rightDia[i+col] = true
				row[i] = true

				backtrack(col + 1)

				row[i] = false
				rightDia[i+col] = false
				leftDia[i-col+n] = false
			}
		}
	}

	backtrack(0)
	return res
}
