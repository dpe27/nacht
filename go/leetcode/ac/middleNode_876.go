package ac

import "hashirama/leetcode/tmp"

func middleNode(head *tmp.ListNode) *tmp.ListNode {
	tail := head
	middle := head
	for tail != nil && tail.Next != nil {
		tail = tail.Next.Next
		middle = middle.Next
	}
	return middle
}
