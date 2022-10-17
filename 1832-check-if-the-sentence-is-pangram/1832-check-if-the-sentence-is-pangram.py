class Solution:
    def checkIfPangram(self, s: str) -> bool:
        n = 0
        if len(s) > 40:
            for c in s:
                n |= (1 << ord(c) - 97)
                if n == 67108863: return True
            return False
        for c in s:
            n |= (1 << ord(c) - 97)
        return n == 67108863
    