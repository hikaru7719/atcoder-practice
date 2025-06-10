n = int(input())
a = list(map(int, input().split()))


result = 0

for i in range(0, 101):
    count = 0
    for aa in a:
        if i <= aa:
            count += 1
    if count >= i:
        result = max(result, i)

print(result)
