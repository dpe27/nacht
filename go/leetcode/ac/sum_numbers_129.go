package ac

func sumNumbers(root *TreeNode) int {
	res := 0
	var traversal func(root *TreeNode, num int)
	traversal = func(root *TreeNode, num int) {
		if root == nil {
			return
		}

		if root.Left == nil && root.Right == nil {
			res += num*10 + root.Val
			return
		}

		traversal(root.Left, num*10+root.Val)
		traversal(root.Right, num*10+root.Val)
	}

	traversal(root, 0)
	return res
}
