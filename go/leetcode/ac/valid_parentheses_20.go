package ac

func isValid20(s string) bool {
	var n int
	stk := []byte{}
	for i := 0; i < len(s); i++ {
		if s[i] == '(' || s[i] == '[' || s[i] == '{' {
			stk = append(stk, s[i])
			continue
		} 
	
		n = len(stk)
		if (n < 1) {
			return false
		}

		if (s[i] == ')' && stk[n-1] == '(') || (s[i] == ']' && stk[n-1] == '[') || (s[i] == '}' && stk[n-1] == '{') {
			stk = stk[:n-1]
		} else {
			return false
		}
	}

	return len(stk) == 0
}