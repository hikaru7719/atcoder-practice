n = int(input())
s = list(map(int, input().split()))

min_val = 10**6
d: dict[int, int] = {}
for i in range(n):
    if s[i] in d:
        min_val = min(min_val, i - d[s[i]] + 1)

    d[s[i]] = i

if min_val == 10**6:
    print(-1)
else:
    print(min_val)
