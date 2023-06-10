n = int(input())

p = n // 5
r = n % 5

if 2 < r:
    print((p+1) * 5)
else:
    print(p * 5)
