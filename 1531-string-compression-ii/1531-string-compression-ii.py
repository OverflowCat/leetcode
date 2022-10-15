class Solution:
    def getLengthOfOptimalCompression(self, s: str, k: int) -> int:
        diglen = [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4]
        n = len(s)
        f = [[127 for _ in range(k + 1)] for __ in range(n + 1)]
        f[0][0] = 0
        for i in range(1, n + 1):
            for j in range(min(i, k) + 1):
                if j > 0: f[i][j] = f[i - 1][j - 1]
                same, diff = 0, 0
                for i_ in range(i, 0, -1):
                    if s[i_ - 1] == s[i - 1]:
                        same += 1
                        f[i][j] = min(f[i][j], f[i_ - 1][j - diff] + diglen[same])
                    else:
                        diff += 1
                        if diff > j: break
        return f[n][k]