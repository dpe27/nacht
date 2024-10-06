package ac

import "sort"

func longestCommonPrefix(strs []string) string {
	var res string
	sort.Strings(strs)

	first, last := strs[0], strs[len(strs)-1]
	for i := 0; i < min(len(first), len(last)); i++ {
		if first[i] != last[i] {
			return res
		}
		res += string(first[i])
	}

	return res
}
