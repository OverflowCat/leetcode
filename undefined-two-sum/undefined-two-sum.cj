import std.collection.*

class Solution {
    func twoSum(nums: Array<Int64>, target: Int64): Array<Int64> {
        let map = HashMap<Int64, Int64>()
        for (i in 0..nums.size) {
            let n = nums[i]
            if (map.contains(n)) {
                return [i, map[n]]
            }
            map[target - n] = i
        }
        []
    }
}