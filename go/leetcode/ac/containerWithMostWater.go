package ac

func maxArea(height []int) int {
	n := len(height)
	start, end := 0, n-1
	area := 0
	maxWidth := n - 1

	for width := maxWidth; width > 0; width-- {
		if height[start] > height[end] {
			area = max(area, width*height[end])
			end--
		} else {
			area = max(area, width*height[start])
			start++
		}
	}

	return area
}
