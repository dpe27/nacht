package ac

import "strconv"

//type TreeNode struct {
//	Val   int
//	Left  *TreeNode
//	Right *TreeNode
//}

func tree2str(root *TreeNode) string {
	if root == nil {
		return ""
	}
	if root.Left == nil && root.Right == nil {
		return strconv.Itoa(root.Val)
	}
	leftStr := tree2str(root.Left)
	rightStr := tree2str(root.Right)
	if root.Left != nil && root.Right == nil {
		return strconv.Itoa(root.Val) + "(" + leftStr + ")"
	}
	if root.Left == nil && root.Right != nil {
		return strconv.Itoa(root.Val) + "()(" + rightStr + ")"
	}
	return strconv.Itoa(root.Val) + "(" + leftStr + ")(" + rightStr + ")"
}
