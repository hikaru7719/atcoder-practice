n, d = map(int, input().split())
l = []
for i in range(n):
    x, y = map(int, input().split())
    l.append((x, y))

for i in range(d):
    z = i + 1
    result = -1
    for x, y in l:
        result = max(result, x * (y + z))
    print(result)
