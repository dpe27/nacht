package ac

type Node struct {
	Val   int
	Left  *Node
	Right *Node
	Next  *Node
}

func connect(root *Node) *Node {
	if root == nil {
		return nil
	}
	prev := root
	curr := new(Node)
	for prev.Left != nil {
		curr = prev
		for curr != nil {
			curr.Left.Next = curr.Right
			if curr.Next != nil {
				curr.Right.Next = curr.Next.Left
			}
			curr = curr.Next
		}
		prev = prev.Left
	}
	return root
}
