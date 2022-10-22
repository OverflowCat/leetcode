class Solution:
    def minWindow(self, s: str, t: str) -> str:
        d = defaultdict(lambda: 0)
        for c in t: d[c] += 1
        d = d.items()
        t, length = set(t), len(s)
        win = defaultdict(lambda: 0)
        l, r, minc, minv = 0, 0, length + 1, (0, -1)
        while l < length:
            if all([win[k] >= v for k, v in d]):
                if r - l < minc:
                    minc, minv = r - l, (l, r)
                c = s[l]
                if c in win: win[c] -= 1
                l += 1
            elif r == length:
                break
            else:
                if s[r] in t: win[s[r]] += 1
                r += 1
            # print(s[l:r])
        if all([win[k] >= v for k, v in d]) and r - l < minc:
            minc, minv = r - l, (l, r)
        if length + 1 == minc: return ""
        # print({minc}, {minv}, {l}, {r})
        return s[minv[0]:minv[1]]