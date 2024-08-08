package ac

func generateParenthesis(n int) []string {
	var res []string
	var backtrack func(int, int, []byte)

	backtrack = func(l, r int, combination []byte) {
		if r < l {
			return
		}

		if l < 0 || r < 0 {
			return
		}

		if l == 0 && r == 0 {
			res = append(res, string(combination))
			return
		}

		backtrack(l-1, r, append(combination, '('))
		backtrack(l, r-1, append(combination, ')'))
	}
	backtrack(n, n, nil)
	return res
}
