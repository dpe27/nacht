package ac

func flatten(root *TreeNode) {
	var prev *TreeNode
	var build func(root *TreeNode)
	build = func(root *TreeNode) {
		if root == nil {
			return
		}
		build(root.Right)
		build(root.Left)
		root.Right = prev
		root.Left = nil
		prev = root
	}
	build(root)
}
