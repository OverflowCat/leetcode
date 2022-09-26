class Solution:
    def equationsPossible(self, s):
        p = [-1] * 26
        def find(x):
            return x if p[x] == -1 else find(p[x])
        for e in s:
            if e[1] == '!': continue
            i, j = ord(e[0]) - 97, ord(e[3]) - 97
            if i == j: continue
            x, y = find(i), find(j)
            if x != y: p[y] = x
        for e in s:
            if e[1] == '=': continue
            i, j = ord(e[0]) - 97, ord(e[3]) - 97
            if i == j: return False
            if find(i) == find(j): return False
        return True