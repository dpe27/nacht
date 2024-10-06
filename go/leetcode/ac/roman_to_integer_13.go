package ac

func romanToInt(s string) int {
	runes := []rune(s)
	n := len(runes)

	roman := map[string]int{
		"I": 1,
		"V": 5,
		"X": 10,
		"L": 50,
		"C": 100,
		"D": 500,
		"M": 1000,
	}

	res := roman[string(runes[n-1])]
	for i := n - 2; i >= 0; i-- {

		if roman[string(runes[i])] < roman[string(runes[i+1])] {
			res -= roman[string(runes[i])]
		} else {
			res += roman[string(runes[i])]
		}
	}

	return res
}
