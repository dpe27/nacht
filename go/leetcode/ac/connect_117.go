package ac

func connect117(root *Node) *Node {
	if root == nil {
		return nil
	}
	q := []*Node{root}
	for len(q) > 0 {
		sl := len(q)
		var next *Node
		for i := 0; i < sl; i++ {
			curr := q[0]
			q = q[1:]
			if curr == nil {
				break
			}
			curr.Next = next
			next = curr

			if (curr.Right != nil) {
				q = append(q, curr.Right)
			}

			if (curr.Left != nil) {
				q = append(q, curr.Left)
			}
		}
	}
	return root
}
