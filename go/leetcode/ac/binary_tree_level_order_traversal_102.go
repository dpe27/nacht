package ac

func levelOrder(root *TreeNode) [][]int {
	var res [][]int
	if root == nil {
		return res
	}
	var q []*TreeNode
	q = append(q, root)
	for len(q) > 0 {
		levelSize := len(q)
		var subarray []int
		for i := 0; i < levelSize; i++ {
			node := q[0]
			q = q[1:]
			subarray = append(subarray, node.Val)
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
		res = append(res, subarray)
	}
	return res
}
