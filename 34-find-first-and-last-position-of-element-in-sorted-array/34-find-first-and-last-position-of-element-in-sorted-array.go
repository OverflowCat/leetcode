func searchRange(nums []int, target int) []int {
    length := len(nums)
    var l, r int
    foundl := false
    foundr := false
    for i := 0; length > i; i++ {
        if foundl {
            if nums[i] != target {
                r = i - 1
                foundr = true
                break
            }
        } else if nums[i] == target {
            l = i
            foundl = true
        }
    }
    if !foundl {
        l = -1
        r = -1
    } else if !foundr {
        r = length - 1
    }
    
    res := []int{l, r}
    return res
}
