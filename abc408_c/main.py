n, m = map(int, input().split())


nn = [0] * n
for i in range(m):
    l, r = map(int, input().split())
    l -= 1
    nn[l] += 1
    if r < n:
        nn[r] -= 1

result = 2 * 10**5 + 1
current = 0
for i in range(n):
    current += nn[i]
    result = min(result, current)

print(result)
