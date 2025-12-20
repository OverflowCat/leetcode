import std.collection.*

class Solution {
    func minDeletionSize(输入: Array<String>): Int64 {
        var 删除数量 = 0
        // let 列数 = 输入[0].count
        for ((列索引, _) in 输入[0] |> enumerate) {
            for (行索引 in 1..输入.size) {
                match (输入[行索引][列索引].compare(输入[行索引 - 1][列索引])) {
                    case Ordering.LT =>
                        删除数量 += 1
                        println(列索引)
                        break
                    case _ => continue
                }
            }
        }
        删除数量
    }
}
