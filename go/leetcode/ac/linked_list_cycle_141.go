package ac

import "hashirama/leetcode/tmp"

func hasCycle(head *tmp.ListNode) bool {
	if head == nil {
		return false
	}
	slow, fast := head, head
	for slow.Next != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
		if slow == fast {
			return true
		}
	}
	return false
}
