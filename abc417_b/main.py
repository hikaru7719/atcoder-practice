n, m = map(int, input().split())
a = list(map(int, input().split()))
b = list(map(int, input().split()))


b.sort()

ai = 0
bi = 0
result = []
while True:
    if ai == n:
        break
    if bi == m:
        for i in range(ai, n):
            result.append(a[i])
        break
    if a[ai] < b[bi]:
        result.append(a[ai])
        ai += 1
    elif a[ai] == b[bi]:
        ai += 1
        bi += 1
    else:
        bi += 1


print(" ".join(map(str, result)))
