from itertools import permutations
from math import sqrt

n, s, t = list(map(int, input().split()))
input = [list(map(int, input().split())) for _ in range(n)]


def dist(a: tuple[int, int], b: tuple[int, int], speed: int) -> float:
    return sqrt((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2) / speed


ans: float = 2**31 - 1

for p in permutations(range(n)):
    for i in range(1 << n):
        sum: float = 0
        first = (0, 0)
        target = [i & (1 << j) for j in range(n)]
        for k in range(n):
            cur = p[k]
            if target[k]:
                second = (input[cur][0], input[cur][1])
                third = (input[cur][2], input[cur][3])
            else:
                second = (input[cur][2], input[cur][3])
                third = (input[cur][0], input[cur][1])
            sum += dist(first, second, s)
            sum += dist(second, third, t)
            first = third
        ans = min(ans, sum)

print(ans)
