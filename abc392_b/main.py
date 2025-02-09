n, m = map(int, input().split())
a = list(map(int, input().split()))

x = {i: False for i in range(1, n + 1)}
for aa in a:
    x[aa] = True

result = []
for xx in x:
    if not x[xx]:
        result.append(xx)

print(len(result))
print(" ".join(map(str, result)))
