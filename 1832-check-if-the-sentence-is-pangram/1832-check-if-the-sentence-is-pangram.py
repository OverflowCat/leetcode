class Solution:
    def checkIfPangram(self, sentence: str) -> bool:
        n = 0
        for c in sentence:
            n |= (1 << ord(c) - 97)
            if n == 67108863: return True
        return False