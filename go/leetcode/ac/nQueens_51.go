package ac

func solveNQueens(n int) [][]string {
	leftDia := make([]int, 2*n)
	for i := range leftDia {
		leftDia[i] = 0
	}

	rightDia := make([]int, 2*n)
	for i := range rightDia {
		rightDia[i] = 0
	}

	row := make([]int, n)
	for i := range row {
		row[i] = 0
	}

	board := make([][]int, n)
	for i := range board {
		board[i] = make([]int, n)
	}
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			board[i][j] = 0
		}
	}

	var res [][]string
	var backtracking func(board [][]int, col int) bool

	backtracking = func(board [][]int, col int) bool {
		if col >= n {
			var ans []string
			for i := 0; i < n; i++ {
				rowStr := ""
				for j := 0; j < n; j++ {
					if board[i][j] == 1 {
						rowStr += "Q"
					} else {
						rowStr += "."
					}
				}
				ans = append(ans, rowStr)
			}
			res = append(res, ans)
			return true
		}

		hasSol := false
		for i := 0; i < n; i++ {
			if leftDia[i-col+n] == 0 && rightDia[i+col] == 0 && row[i] == 0 {
				board[i][col] = 1
				leftDia[i-col+n] = 1
				rightDia[i+col] = 1
				row[i] = 1

				if backtracking(board, col+1) {
					hasSol = true
				}

				board[i][col] = 0
				leftDia[i-col+n] = 0
				rightDia[i+col] = 0
				row[i] = 0
			}
		}
		return hasSol
	}

	backtracking(board, 0)
	return res
}
