n = int(input())
a = list(map(int, input().split()))

sum = 0
for i in range(n):
    if i % 2 == 0:
        sum += a[i]
print(sum)
