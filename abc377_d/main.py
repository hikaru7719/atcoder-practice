n, m = map(int, input().split())
d = [1 for i in range(m + 1)]

for _ in range(n):
    l, r = map(int, input().split())
    d[r] = max(d[r], l + 1)

for r in range(1, m + 1):
    d[r] = max(d[r], d[r - 1])


ans = 0
for r in range(1, m + 1):
    ans += r - d[r] + 1

print(ans)
