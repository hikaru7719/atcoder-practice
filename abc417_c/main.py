n = int(input())
a = list(map(int, input().split()))


mm = {}
count = 0
for i in range(n):
    ii = i + 1
    ai = a[i]

    target = -(ai - ii)
    if target in mm:
        count += mm[target]

    num = ai + ii
    if num not in mm:
        mm[num] = 1
    else:
        mm[num] += 1
print(count)
