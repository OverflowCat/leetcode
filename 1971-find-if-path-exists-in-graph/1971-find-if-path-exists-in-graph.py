class Solution:
    def validPath(self, n: int, edges: List[List[int]], source: int, destination: int) -> bool:
        nodes = defaultdict(list)
        for x in edges:
          nodes[x[0]].append(x[1])
          nodes[x[1]].append(x[0])
        visited = [False] * n
        def f(node):
          nonlocal destination
          if node == destination:
            return True
          if not visited[node]:
            visited[node] = True
            for next_node in nodes[node]:
              if f(next_node): return True
          return False
        return f(source)
