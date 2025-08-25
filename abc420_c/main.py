n, q = map(int, input().split())

a = list(map(int, input().split()))
b = list(map(int, input().split()))

result = 0
min_list = [0] * n
for i in range(n):
    result += min(a[i], b[i])
    min_list[i] = min(a[i], b[i])

for _ in range(q):
    c, x, v = input().split()
    xx = int(x)
    vv = int(v)
    if c == "A":
        a[xx - 1] = vv
    else:
        b[xx - 1] = vv

    new_min = min(a[xx - 1], b[xx - 1])
    result += new_min - min_list[xx - 1]
    min_list[xx - 1] = new_min
    print(result)
