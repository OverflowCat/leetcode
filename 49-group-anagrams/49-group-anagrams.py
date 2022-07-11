class Solution:
    def groupAnagrams(self, strs):
        map = {}
        for s in strs:
            o = ""
            for c in "abcdefghijklmnopqrstuvwxyz":
                o += str(s.count(c)) + c
            if o not in map:
                map[o] = [s]
            else:
                map[o].append(s)
        return list(map.values())