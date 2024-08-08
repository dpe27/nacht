package ac

import "hashirama/leetcode/tmp"

func deleteMiddle(head *tmp.ListNode) *tmp.ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	tail := head
	middle := head
	prevMiddle := &tmp.ListNode{Next: middle}

	for tail != nil && tail.Next != nil {
		tail = tail.Next.Next
		middle = middle.Next
		prevMiddle = prevMiddle.Next
	}

	prevMiddle.Next = middle.Next
	return head
}
