n = int(input())
k = list(map(int, input().split()))

ans = 10**9
for i in range(1 << (n + 1)):
    a = 0
    b = 0
    for j in range(n):
        if i & (1 << j):
            a += k[j]
        else:
            b += k[j]
    ans = min(ans, max(a, b))

print(ans)
