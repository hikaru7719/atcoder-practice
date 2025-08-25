n, m = map(int, input().split())

s = []
for i in range(n):
    s.append(list(input()))

count = [0] * n

for i in range(m):
    x = []
    y = []
    for j in range(n):
        if s[j][i] == "0":
            x.append(j)
        else:
            y.append(j)
    if not x or not y:
        for j in x + y:
            count[j] += 1
    elif len(x) > len(y):
        for j in y:
            count[j] += 1
    else:
        for j in x:
            count[j] += 1

score = max(c for c in count)

result = []
for i, c in enumerate(count):
    if c == score:
        result.append(str(i + 1))

print(" ".join(result))
