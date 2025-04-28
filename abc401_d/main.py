n, k = map(int, input().split())

a = [0 for _ in range(n + 1)]
s = [0 for _ in range(n + 1)]
a[0] = 1
s[0] = 1

for i in range(1, n + 1):
    if i < k:
        a[i] = 1 % (10**9)
    elif i == k:
        a[i] = s[i - 1] % (10**9)
    else:
        a[i] = (s[i - 1] - s[i - k - 1]) % (10**9)
    s[i] = (s[i - 1] + a[i]) % (10**9)


print(a[n])
