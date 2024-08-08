package ac

import "hashirama/leetcode/tmp"

func swapPairs(head *tmp.ListNode) *tmp.ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	res := &tmp.ListNode{Next: head}
	dummyHead := res
	for head != nil && head.Next != nil {
		prev := head.Next
		head.Next = prev.Next
		prev.Next = head

		dummyHead.Next = prev
		head = head.Next
		dummyHead = dummyHead.Next.Next
	}

	if head != nil && head.Next != nil {
		dummyHead.Next = head.Next
	}

	return res.Next
}
