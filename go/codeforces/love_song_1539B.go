package main

import "fmt"

func main() {
	var n, q int
	var s string
	fmt.Scan(&n, &q, &s)
	dp := make([]int, n+1)
	for i := 1; i <= n; i++ {
		dp[i] = dp[i-1] + int(s[i-1]-'a') + 1
	}
	for q > 0 {
		var l, r int
		fmt.Scan(&l, &r)
		fmt.Println(dp[r] - dp[l-1])
		q--
	}
}
