```py
@cached(cache={})
def climb(n: int) -> int:
if n < 3:
return n
return climb(n - 1) + climb(n - 2)
```