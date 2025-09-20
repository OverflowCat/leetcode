import kotlin.math.abs

class Solution {
    fun reverse(x: Int): Int {
        var res: Int = 0
        var num = abs(x)
        while (num != 0) {
            val last = res
            res *= 10
            res += num % 10
            if (res / 10 != last) {
                return 0
            }
            num /= 10
        }
        if (x < 0) {
            res = -res
        }
        return res
    }
}