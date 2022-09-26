class Solution:
    def equationsPossible(self, s):
        p = {}
        def find(x):
            return x if x not in p else find(p[x])
        for e in s:
            if e[1] == '!': continue
            if e[0] == e[3]: continue
            x, y = find(e[0]), find(e[3])
            if x != y: p[y] = x
        for e in s:
            if e[1] == '=': continue
            if e[0] == e[3]: return False
            if find(e[0]) == find(e[3]): return False
        return True