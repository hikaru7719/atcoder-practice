n, t = map(int, input().split())
c = list(map(int, input().split()))
r = list(map(int, input().split()))
t2 = c[0]

result1 = -1
result1_max = -1
result2 = -1
result2_max = -1

for i in range(0, n):
    ci, ri = c[i], r[i]
    if ci == t:
        if result1_max < ri:
            result1_max = ri
            result1 = i
    if ci == t2:
        if result2_max < ri:
            result2_max = ri
            result2 = i

if result1 == -1:
    print(result2+1)
else:
    print(result1+1)