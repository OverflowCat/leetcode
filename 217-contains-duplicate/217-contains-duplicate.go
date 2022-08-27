type void struct{}
var one void

func containsDuplicate(nums []int) bool {
    set := make(map[int]void)
    for _, num := range nums {
        if _, ok := set[num]; ok {
            return true
        }
        set[num] = one
    }
    return false
}
