n = int(input())

a = list(map(int, input().split()))

b = a[1] / a[0]

result = True
for i in range(1, n):
    if a[i] != b * a[i - 1]:
        result = False

if result:
    print("Yes")
else:
    print("No")
