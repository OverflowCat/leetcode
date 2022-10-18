class Solution:
    def countAndSay(self, n: int) -> str:
        ns = "1"
        for _ in range(1, n):
            count = 1
            output = ""
            for j in range(0, len(ns) - 1):
                if ns[j] != ns[j + 1]:
                    output += str(count) + ns[j]
                    count = 1
                else:
                    count += 1
            output += str(count) + ns[-1]
            ns = output
        return ns
