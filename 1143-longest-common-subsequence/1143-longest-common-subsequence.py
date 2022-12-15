import numpy as np
class Solution:
  def longestCommonSubsequence(self, text1: str, text2: str) -> int:
    m, n = len(text1), len(text2)
    a = np.zeros((m+1, n+1), dtype=np.uint16)
    for i in range(m):
      for j in range(n):
        a[i+1][j+1]=a[i][j]+1 if text1[i]==text2[j] else max(a[i+1][j], a[i][j+1])
    return a[m][n]