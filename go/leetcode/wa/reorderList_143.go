package wa

import "hashirama/leetcode/tmp"

func reorderList(head *tmp.ListNode) {
	if head == nil || head.Next == nil {
		return
	}
	var middleList func(head *tmp.ListNode) *tmp.ListNode
	middleList = func(head *tmp.ListNode) *tmp.ListNode {
		tail, middle := head, head
		for tail.Next != nil && tail.Next.Next != nil {
			tail = tail.Next.Next
			middle = middle.Next
		}
		return middle
	}

	var reverse func(head *tmp.ListNode) *tmp.ListNode
	reverse = func(head *tmp.ListNode) *tmp.ListNode {
		curr := head
		prev := new(tmp.ListNode)
		for curr != nil {
			next := curr.Next
			curr.Next = prev
			prev = curr
			curr = next
		}
		return prev
	}

	middleNode := middleList(head)
	nextMiddle := middleNode.Next
	middleNode.Next = nil

	list1 := head
	list2 := reverse(nextMiddle)
	for list2 != nil {
		next1 := list1.Next
		next2 := list2.Next
		list1.Next = list2
		list2.Next = next1
		list1 = next1
		list2 = next2
	}
}
