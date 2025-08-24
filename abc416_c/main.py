n, k, x = map(int, input().split())

nn = [i for i in range(n)]
ss: list[str] = []

for _ in range(n):
    s = input()
    ss.append(s)

count = 0
h: list[str] = []

for i in range(n):
    if k == 1:
        h.append(ss[i])
    for i2 in range(n):
        if k == 2:
            h.append(ss[i] + ss[i2])
            continue
        for i3 in range(n):
            if k == 3:
                h.append(ss[i] + ss[i2] + ss[i3])
                continue
            for i4 in range(n):
                if k == 4:
                    h.append(ss[i] + ss[i2] + ss[i3] + ss[i4])
                    continue
                for i5 in range(n):
                    if k == 5:
                        h.append(ss[i] + ss[i2] + ss[i3] + ss[i4] + ss[i5])
                        continue
h.sort()


# print(h)
# print(len(h))
print(h[x - 1])
