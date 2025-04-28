n, m = map(int, input().split())

result = 0
inf = 10**9
infFlag = False
for i in range(m + 1):
    result += n**i
    if inf < result:
        infFlag = True
        break

if infFlag:
    print("inf")
else:
    print(result)
