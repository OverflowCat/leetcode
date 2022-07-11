class Solution:
    def trap(self, height) -> int:
        height = [0] + height + [0]
        lhighest = []
        maxv = 0
        for x in height:
            if x > maxv:
                maxv = x
            lhighest.append(maxv)

        rhighest = []
        maxv = 0
        for x in height[::-1]:
            if x > maxv:
                maxv = x
            rhighest.append(maxv)
        rhighest = rhighest[::-1]
        sum = 0
        for i in range(len(height)):
            sum += min(lhighest[i], rhighest[i]) - height[i]
        return sum