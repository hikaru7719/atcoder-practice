n, m = map(int, input().split())
a: list[dict[int, bool]] = [{} for _ in range(n)]
count = 0
for _ in range(m):
    b, c = map(int, input().split())
    if c - 1 in a[b - 1]:
        count += 1
        continue
    else:
        a[b - 1][c - 1] = True
    if b - 1 in a[c - 1]:
        count += 1
        continue
    else:
        a[c - 1][b - 1] = True

print(count)
