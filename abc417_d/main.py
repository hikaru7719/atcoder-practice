import bisect
from itertools import accumulate

N = int(input())

motivation = []
for _ in range(N):
    p, a, b = list(map(int, input().split()))
    motivation.append((p, a, b))

M = max(p + a for p, a, b in motivation)
dp = [[0] * (M + 1) for _ in range(N + 1)]

for j in range(M + 1):
    dp[N][j] = j

for i in range(N - 1, -1, -1):
    p, a, b = motivation[i]
    for j in range(M + 1):
        if j <= p:
            dp[i][j] = dp[i + 1][j + a]
        else:
            dp[i][j] = dp[i + 1][j - min(j, b)]

B_values = [b for p, a, b in motivation]
sum_B = list(accumulate(B_values))


def access(x):
    if x <= M:
        return dp[0][x]
    else:
        need_down_idx = bisect.bisect_left(sum_B, x - M)
        if need_down_idx == N:
            return x - sum_B[-1]
        return dp[need_down_idx + 1][x - min(x, sum_B[need_down_idx])]


Q = int(input())
results = []
for _ in range(Q):
    xi = int(input())
    results.append(str(access(xi)))

print("\n".join(results))
