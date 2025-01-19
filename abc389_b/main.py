import math

i = int(input())

n = 1
while True:
    if math.factorial(n) == i:
        print(n)
        break
    else:
        n += 1
