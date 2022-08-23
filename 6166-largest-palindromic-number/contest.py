num = "0001"

count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
for x in num:
    count[int(x)] += 1

奇数个的数字 = [i for i, x in enumerate(count) if x % 2 == 1]
最大中间位 = str(max(奇数个的数字)) if len(奇数个的数字) != 0 else ""

def reverseEnum(data: list):
    for i in range(len(data)-1, -1, -1):
        yield (i, data[i])

halfseq = ""

for i, x in reverseEnum(count):
    halfseq += str(i) * (x // 2)

leading_zero_count = 0
for x in halfseq:
    if x == '0':
        leading_zero_count += 1
    else:
        break
halfseq = halfseq[leading_zero_count:]

res = int(halfseq + 最大中间位 + halfseq[::-1])
print(res)