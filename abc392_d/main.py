import itertools

n = int(input())
a = []
count_a = []
for _ in range(n):
    z = list(map(int, input().split()))
    d: dict[int, int] = {}
    for zz in z:
        if zz in d:
            d[zz] += 1
        else:
            d[zz] = 1
    count_a.append(d)
    a.append(z)


p = 0.00
for cc, dd in itertools.combinations([i for i in range(n)], 2):
    x = a[cc]
    y = a[dd]

    all = x[0] * y[0]
    count = 0
    set_x = set(x[1:])
    set_y = set(y[1:])
    set_xy = set_x & set_y
    for xx in set_xy:
        count += count_a[cc][xx] * count_a[dd][xx]
    p = max(p, count / all)

print(p)
