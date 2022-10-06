from sortedcontainers import SortedDict

class TimeMap:

    def __init__(自个儿):
        自个儿.数据 = defaultdict(SortedDict)

    def set(自个儿, 键, 值, 时间戳):
        自个儿.数据[键][时间戳] = 值

    def get(自个儿, 键, 时间戳):
        索引 = 自个儿.数据[键].bisect_right(时间戳)
        if 索引 == 0: return ""
        最大时间戳 = 自个儿.数据[键].keys()[索引 - 1]
        return 自个儿.数据[键][最大时间戳]
