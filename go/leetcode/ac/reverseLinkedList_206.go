package ac

import "hashirama/leetcode/tmp"

func reverseList(head *tmp.ListNode) *tmp.ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	curr := head
	var prev *tmp.ListNode
	var next *tmp.ListNode
	for curr != nil {
		next = curr.Next
		curr.Next = prev
		prev = curr
		curr = next
	}
	return prev
}
