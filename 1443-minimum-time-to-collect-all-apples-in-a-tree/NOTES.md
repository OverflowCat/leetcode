e = None
c = 0
def f(i: int) -> int:
v = tree[i]
if v == []:
return 2 if ap[i] else 0
res = f(v[0])
if len(v) == 2: res += f(v[1])
return 0 if res == 0 else res + 2
return f(0)
```
​
```python
class Solution:
def minTime(self, n: int, e: List[List[int]], ap: List[bool]) -> int:
visited = [False] * n
visited[0] = True
tree = [[] for _ in range(n)]
for a, b in e:
if visited[a]:
tree[a].append(b)
visited[b] = True
else:
tree[b].append(a)
visited[a] = True
e = None
c = 0
def f(i: int) -> int:
v = tree[i]
if v == []:
return 2 if ap[i] else 0
res = f(v[0])
if len(v) == 2: res += f(v[1])
if res == 0 and not ap[i]: return 0
return res + 2
res = f(0)
return res if res == 0 else res - 2