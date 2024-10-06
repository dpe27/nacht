package ac

func strStr28(haystack string, needle string) int {
	m, n, j := len(haystack), len(needle), 0
	if n > m {
		return -1
	}

	for i := 0; i <= m-n; i++ {
		j = 0
		for j < n && haystack[i+j] == needle[j] {
			j++
		}

		if j == n {
			return i
		}
	}

	return -1
}
