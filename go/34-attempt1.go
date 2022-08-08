func searchRange(nums []int, target int) []int {
    length := len(nums)
    var l, r int
    flag := false
    for i := 0; ; i++ {
        if flag {
            if i == length || nums[i] != target {
                r = i - 1
                break
            }
        } else if nums[i] == target {
            l = i
            flag = true
        }
    }
    var res []int
    res = append(res, l)
    res = append(res, r)
    return res
}
