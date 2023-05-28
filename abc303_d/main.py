x, y, z = map(int, input().split())
s = input()


dp = [[10 ** 20 for _ in range(len(s) + 1)] for _ in range(2)]
dp[0][0] = 0

for i in range(1, len(s) + 1):
    if s[i-1] == "a":
        dp[0][i] = min(dp[0][i-1] + x, dp[1][i-1] + z + x)
        dp[1][i] = min(dp[0][i-1] + z + y, dp[1][i - 1] + y) 
    elif s[i-1] == "A":
        dp[0][i] = min(dp[0][i-1] + y, dp[1][i-1] + z + y)
        dp[1][i] = min(dp[0][i-1] + z + x, dp[1][i-1] + x)
 
print(min(dp[0][-1], dp[1][-1]))