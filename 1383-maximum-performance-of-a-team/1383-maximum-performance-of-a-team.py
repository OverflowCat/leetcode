class Solution:
    def maxPerformance(self, n: int, speed: List[int], efficiency: List[int], k: int) -> int:
        # 题目要求我们最优化「速度和」和「效率最小值」的乘积。变化的量有两个，一个是「速度」，一个是「效率」，这看起来有些棘手。我们不妨采用「动一个，定一个」的策略——即我们可以枚举效率的最小值，在所有效率大于该最小值的工程师中选取不超过 k - 1 个，让他们的速度和最大。
        s = 0
        ans = 0
        h = []
        arr = sorted(zip(speed, efficiency), key=lambda x: x[1], reverse=True) # 具体地，我们可以对工程师先按照「效率」从高到低的规则排序，然后从前往后枚举这个序列中的元素作为最小值，这样可以保证前面的元素的效率都比当前这个工程师高
        for x in arr:
            s += x[0]
            heappush(h, x[0]) # 然后维护一个以「速度」为关键字的小根堆，存放前面已经枚举过的元素中速度前 k - 1 大的，动态维护这个堆的速度和，一轮枚举后，我们可以得到乘积最大值。
            if len(h) > k: s -= heappop(h) # 当堆内的元素超过 k - 1 的时候，我们可以从堆顶 pop 掉比较小的元素，保证最大的 k - 1 个元素还在堆中。
            ans = max(s * x[1], ans)
        return ans % (10**9 + 7)
