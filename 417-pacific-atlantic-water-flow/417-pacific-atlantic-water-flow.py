class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        m = len(heights)
        n = len(heights[0])
        pacific = set()

        def pbfs(x:int, y:int, prev: int):
            if (x, y) in pacific:
                return
            curr = heights[x][y]
            if curr < prev:
                return
            pacific.add((x, y))
            if x > 0:
                pbfs(x - 1, y, curr)
            if y > 0:
                pbfs(x, y - 1, curr)
            if x < m - 1:
                pbfs(x + 1, y, curr)
            if y < n - 1:
                pbfs(x, y + 1, curr)
                
        atlantic = set()
        def abfs(x:int, y:int, prev: int):
            if (x, y) in atlantic:
                return
            curr = heights[x][y]
            if curr < prev:
                return
            atlantic.add((x, y))
            if x > 0:
                abfs(x - 1, y, curr)
            if y > 0:
                abfs(x, y - 1, curr)
            if x < m - 1:
                abfs(x + 1, y, curr)
            if y < n - 1:
                abfs(x, y + 1, curr)
        for i in range(m):
            pbfs(i, 0, 0)
        for j in range(1, n):
            pbfs(0, j, 0)
        
        for i in range(m):
            abfs(i, n - 1, 0)
        for j in range(0, n - 1):
            abfs(m - 1, j, 0)
            
        return [list(p) for p in atlantic if p in pacific]
