class Solution:
    def minimizeXor(self, num1: int, num2: int) -> int:
        ones = num2.bit_count()
        num1b = bin(num1)[2:]
        x = 0
        pown = len(num1b) - 1
        for bit in num1b:
            if bit == '1': # 1 XOR 1 == 0
                if ones == 0:
                    break
                else:
                    ones -= 1
                x += 1 << pown
            pown -= 1
        i = 0
        while i < ones:
            if (x >> i) % 2 == 0:
                x += 1 << i
            i += 1
        return x
