package ac

import (
	"hashirama/leetcode/tmp"
	"slices"
)

func buildTree106(inorder []int, postorder []int) *tmp.TreeNode {
	var build func(inorder []int, postorder []int) *tmp.TreeNode
	build = func(inorder []int, postorder []int) *tmp.TreeNode {
		n := len(postorder)
		if len(inorder) == 0 {
			return nil
		}
		root := &tmp.TreeNode{Val: postorder[n-1]}
		rootIdx := slices.Index(inorder, root.Val)
		root.Left = build(inorder[:rootIdx], postorder[:rootIdx])
		root.Right = build(inorder[rootIdx+1:], postorder[rootIdx:n-1])
		return root
	}
	return build(inorder, postorder)
}
