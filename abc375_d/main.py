s = list(input())
n = len(s)

sum = [[0 for __ in range(n + 1)] for _ in range(26)]

for i in range(n):
    for j in range(26):
        sum[j][i + 1] = sum[j][i]
    sum[ord(s[i]) - ord("A")][i + 1] += 1

ans = 0
for i in range(1, n - 1):
    for j in range(26):
        left = sum[j][i]
        right = sum[j][n] - sum[j][i + 1]
        ans += left * right

print(ans)
