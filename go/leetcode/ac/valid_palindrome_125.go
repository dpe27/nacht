package ac

import (
	"strings"
	"unicode"
)

func isPalindrome125(s string) bool {
	s = strings.ToLower(s)
	runes :=  []rune{}
	for _, c := range s {
		if unicode.IsLetter(c) || unicode.IsDigit(c) {
			runes = append(runes, c)
		}
	}

	s = string(runes)
	for i := 0; i < len(s) / 2; i++ {
		if s[i] != s[len(s)-1-i] {
			return false
		}
	}

	return true
}