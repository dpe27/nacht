package ac

func minimumTotal(triangle [][]int) int {
	n := len(triangle)

	for i := 1; i < n; i++ {
		for j := 0; j <= i; j++ {
			if j == 0 {
				triangle[i][j] += triangle[i-1][j]
				continue
			}

			if j == i {
				triangle[i][j] += triangle[i-1][j-1]
				continue
			}

			triangle[i][j] += min(triangle[i-1][j-1], triangle[i-1][j])
		}
	}

	res := triangle[n-1][0]
	for i := 0; i < n; i++ {
		res = min(res, triangle[n-1][i])
	}

	return res
}
