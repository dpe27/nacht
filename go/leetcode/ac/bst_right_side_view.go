package ac

func rightSideView(root *TreeNode) []int {
	if root == nil {
		return nil
	}

	res := []int{root.Val}
	q := []*TreeNode{root}

	for len(q) > 0 {
		ls := len(q)
		for i := 0; i < ls; i++ {
			curr := q[0]
			q = q[1:]

			if curr.Left != nil {
				q = append(q, curr.Left)
			}

			if curr.Right != nil {
				q = append(q, curr.Right)
			}
		}
		if len(q) > 0 {
			res = append(res, q[len(q)-1].Val)
		}
	}

	return res
}

func rightSideView2(root *TreeNode) []int {
	maxLevel := -1
	res := make([]int, 0)
	var solve func(*TreeNode, int)
	solve = func(root *TreeNode, level int) {
		if root == nil {
			return
		}

		if maxLevel < level {
			maxLevel = level
			res = append(res, root.Val)
		}

		solve(root.Right, level+1)
		solve(root.Left, level+1)
	}

	solve(root, 0)
	return res
}
