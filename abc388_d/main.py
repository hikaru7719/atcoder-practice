n = int(input())
a = list(map(int, input().split()))
c = [0] * n
d = [0] * (n + 1)

for i in range(n):
    if i != 0:
        c[i] = c[i - 1] + d[i]
        a[i] += c[i]
    cnt = min(n - i - 1, a[i])
    a[i] -= cnt
    d[i + 1] += 1
    d[min(n, i + cnt + 1)] -= 1
print(" ".join(list(map(str, a))))
