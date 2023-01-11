class Solution:
    def minTime(self, n: int, e: List[List[int]], ap: List[bool]) -> int:
        visited = [False] * n # 这题的树可能是 [[0,1],[2,3],[1,2]]！
        tree = [[] for _ in range(n)] # sort 貌似也不管用，为啥有人说可以
        for a, b in e: # 看评论以前比较水，没有这这些样例
          tree[a].append(b) # 这题的树可能是 [[0,2],[1,2]]！！
          tree[b].append(a) # 所以题目保证输入的 a < b 其实并没有什么用！！
        e = None # 释放内存
        def f(i: int) -> int: # 不需要经过的情况返回 0
          if visited[i]: return 0 # 实际上不需要单独建树，
          visited[i] = True # 累加的过程本身就是从 0 开始按顺序的
          res = sum(map(f, tree[i])) # 这题的树也可能不是二叉树！！！\U0001f605
          if res == 0 and not ap[i]: return 0 # 注意只有在子节点和本节点上都
          return res + 2 # 没有苹果的时候才返回 9，如果本节点上有则要返回 2 ！！
        ans = f(0) # f 返回值的意思是到达 i 节点所需的时间，所以 f(0) 实际上是从
        return 0 if ans == 0 else ans - 2 # 一个虚拟节点到 0，会有多余的两步!!!!!