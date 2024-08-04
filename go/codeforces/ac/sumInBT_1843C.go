package ac

import "fmt"

func sumInBT_1843C() {
	var t uint16
	fmt.Scan(&t)
	for t > 0 {
		var n int64
		fmt.Scan(&n)
		res := n
		for n != 1 {
			n /= 2
			res += n
		}
		fmt.Println(res)
		t--
	}
}
