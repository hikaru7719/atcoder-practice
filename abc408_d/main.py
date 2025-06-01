def solve():
    n = int(input())
    s = list(input())
    dp = [[0] * (3) for _ in range(n + 1)]
    for i in range(n):
        dp[i + 1][0] = dp[i][0] + (1 if s[i] == "1" else 0)
        dp[i + 1][1] = min(dp[i][0], dp[i][1]) + (1 if s[i] == "0" else 0)
        dp[i + 1][2] = min(dp[i][1], dp[i][2]) + (1 if s[i] == "1" else 0)
    print(min(dp[-1]))


t = int(input())
for _ in range(t):
    solve()
