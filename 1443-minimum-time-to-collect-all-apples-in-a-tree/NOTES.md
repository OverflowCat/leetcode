if v == []:
return 2 if ap[i] else 0
res = f(v[0])
if len(v) == 2: res += f(v[1])
if res == 0 and not ap[i]: return 0
return res + 2
res = f(0)
return res if res == 0 else res - 2
```
​
## Testcases
```
7
[[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]]
[false,false,true,false,true,true,false]
7
[[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]]
[false,false,true,false,false,true,false]
7
[[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]]
[false,false,false,false,false,false,false]
4
[[0,1],[1,2],[0,3]]
[true,true,true,true]
3
[[0,2],[1,2]]
[false,true,false]
4
[[0,2],[0,3],[1,2]]
[false,true,false,false]
8
[[0,1],[1,2],[2,3],[1,4],[2,5],[2,6],[4,7]]
[true,true,false,true,false,true,true,false]
```