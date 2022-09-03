class Solution:
    def numsSameConsecDiff(self, n: int, k: int) -> List[int]:
        res = set()
        def f(generated: int, last: int, remaining_digit_count: int):
            if remaining_digit_count == 0:
                res.add(generated)
                return
            next_num = last + k
            if next_num < 10:
                f(generated * 10 + next_num, next_num, remaining_digit_count - 1)
            next_num = last - k
            if next_num > -1:
                f(generated * 10 + next_num, next_num, remaining_digit_count - 1)
        for i in range(1, 10):
            f(i, i, n - 1)
        return res
