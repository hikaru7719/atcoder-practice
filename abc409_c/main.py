N, L = map(int, input().split())
D = list(map(int, input().split()))

point = [0 for i in range(L)]
point[0] = 1

current = 0
for d in D:
    current = (current + d) % L
    point[current] += 1

length = L // 3

count = 0
if L % 3 != 0:
    print(0)
else:
    for i in range(length):
        p2 = i + length
        p3 = i + 2 * length
        if point[i] > 0 and point[p2] > 0 and point[p3] > 0:
            count += point[i] * point[p2] * point[p3]
    print(count)
