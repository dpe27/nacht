package ac

func exist(board [][]byte, word string) bool {
	closed := make([][]bool, len(board))
	for i := range closed {
		closed[i] = make([]bool, len(board[0]))
		for j := range closed[i] {
			closed[i][j] = false
		}
	}

	xdir := []int{1, 0, -1, 0}
	ydir := []int{0, 1, 0, -1}
	var backtrack func(board [][]byte, idx, row, col int) bool

	backtrack = func(board [][]byte, idx, row, col int) bool {
		if board[row][col] == word[idx] {
			if idx == len(word)-1 {
				return true
			}
			for dir := 0; dir < 4; dir++ {
				nx, ny := row+xdir[dir], col+ydir[dir]
				if nx >= 0 && nx < len(board) && ny >= 0 && ny < len(board[0]) && !closed[nx][ny] {
					closed[nx][ny] = true
					if !backtrack(board, idx+1, nx, ny) {
						closed[nx][ny] = false
					} else {
						return true
					}
				}
			}
		}
		return false
	}

	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			if board[i][j] == word[0] {
				closed[i][j] = true
				if backtrack(board, 0, i, j) {
					return true
				} else {
					closed[i][j] = false
				}
			}
		}
	}

	return false
}
