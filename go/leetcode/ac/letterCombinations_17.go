package ac

func letterCombinations(digits string) []string {
	if len(digits) == 0 {
		return []string{}
	}
	phoneMap := []string{"abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"}
	var output []string
	var backtrack func(string, string)

	backtrack = func(combination string, nextDigits string) {
		if len(nextDigits) == 0 {
			output = append(output, combination)
		} else {
			letters := phoneMap[nextDigits[0]-'2']
			for _, letter := range letters {
				backtrack(combination+string(letter), nextDigits[1:])
			}
		}
	}

	backtrack("", digits)
	return output
}
