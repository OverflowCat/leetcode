class Solution:
    def checkIfPangram(self, sentence: str) -> bool:
        s = set()
        for c in sentence:
            s.add(c)
            if len(s) == 26:
                return True
        return False