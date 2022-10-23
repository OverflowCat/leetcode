class Solution:
    def findErrorNums(self, nums: List[int]) -> List[int]:
        a = set()
        dup = None
        for x in nums:
            if dup is not None or x not in a: a.add(x)
            else: dup = x
        for i in range(1, 10002):
            if i not in a:
                return (dup, i)