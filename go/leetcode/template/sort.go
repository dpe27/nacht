package template

func SelectionSort(nums []int) {
	n := len(nums)
	for i := 0; i < n-1; i++ {
		minIdx := i
		for j := i + 1; j < n; j++ {
			if nums[j] < nums[minIdx] {
				minIdx = j
			}
		}
		if minIdx != i {
			tmp := nums[minIdx]
			nums[minIdx] = nums[i]
			nums[i] = tmp
		}
	}
}

func BubbleSort(nums []int) {
	n := len(nums)
	var swapped bool
	for i := 0; i < n-1; i++ {
		swapped = false
		for j := 0; j < n-i-1; j++ {
			if nums[j] > nums[j+1] {
				nums[j], nums[j+1] = nums[j+1], nums[j]
				swapped = true
			}
		}
		if !swapped {
			break
		}
	}
}

func InsertionSort(nums []int) {
	n := len(nums)
	for i := 1; i < n; i++ {
		key := nums[i]
		j := i - 1
		for j >= 0 && key < nums[j] {
			nums[j+1] = nums[j]
			j--
		}
		nums[j+1] = key
	}
}
