package ac

import (
	"hashirama/leetcode/tmp"
	"slices"
)

func levelOrderBottom(root *tmp.TreeNode) [][]int {
	var res [][]int
	if root == nil {
		return res
	}
	q := []*tmp.TreeNode{root}
	for len(q) > 0 {
		levelSize := len(q)
		var subarr []int
		for i := 0; i < levelSize; i++ {
			curr := q[0]
			q = q[1:]
			subarr = append(subarr, curr.Val)
			if curr.Left != nil {
				q = append(q, curr.Left)
			}
			if curr.Right != nil {
				q = append(q, curr.Right)
			}
		}
		res = append(res, subarr)
	}
	slices.Reverse(res)
	return res
}
