import kotlin.math.abs

class Solution {
    fun reverse(x: Int): Int {
        var res: Int = 0
        var num = x
        while (num != 0) {
            res *= 10
            val rem = num % 10
            if (res > Int.MAX_VALUE / 10 || res < Int.MIN_VALUE) {
                return 0
            }
            res += rem
            num /= 10
        }

        return res
    }
}