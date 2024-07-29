package ac

func grayCode(n int) []int {
	res := []int{0}
	for i := 0; i < n; i++ {
		for j := len(res); j >= 0; j-- {
			res = append(res, res[j]|(1<<i))
		}
	}
	return res
}
