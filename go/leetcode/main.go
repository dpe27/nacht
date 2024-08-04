package main

import "hashirama/leetcode/tmp"

func main() {
	buildListNode := func(arr []int) *tmp.ListNode {
		root := &tmp.ListNode{Val: arr[0]}
		dummy := root
		i := 1
		for i < len(arr) {
			node := &tmp.ListNode{Val: arr[i]}
			dummy.Next = node
			dummy = dummy.Next
			i++
		}
		return root
	}
	buildListNode([]int{1, 2, 3})
}
