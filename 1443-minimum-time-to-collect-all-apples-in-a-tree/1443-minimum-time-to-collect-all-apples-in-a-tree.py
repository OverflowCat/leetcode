class Solution:
    def minTime(self, n: int, e: List[List[int]], ap: List[bool]) -> int:
        visited = [False] * n
        tree = [[] for _ in range(n)]
        for a, b in e: 
          tree[a].append(b)
          tree[b].append(a)
        e = None
        def f(i: int) -> int:
          if visited[i]: return 0
          visited[i] = True
          res = sum(map(f, tree[i])) # 这题的树也可能不是二叉树！！！！
          if res == 0 and not ap[i]: return 0
          return res + 2
        res = f(0)
        return res if res == 0 else res - 2