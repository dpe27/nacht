package ac

import (
	"hashirama/leetcode/tmp"
	"slices"
)

func zigzagLevelOrder(root *tmp.TreeNode) [][]int {
	var res [][]int
	if root == nil {
		return res
	}
	reverse := false
	q := []*tmp.TreeNode{root}
	for len(q) > 0 {
		levelSize := len(q)
		var subarray []int
		for i := 0; i < levelSize; i++ {
			curr := q[0]
			q = q[1:]
			subarray = append(subarray, curr.Val)
			if curr.Left != nil {
				q = append(q, curr.Left)
			}
			if curr.Right != nil {
				q = append(q, curr.Right)
			}
		}
		res = append(res, subarray)
	}
	for i := range res {
		if reverse {
			slices.Reverse(res[i])
		}
		reverse = !reverse
	}
	return res
}
