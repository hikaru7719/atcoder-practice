n = int(input())
p = list(map(int, input().split()))
q = list(map(int, input().split()))

zekken = []
num = []

for i in range(n):
    pp = p[i]
    qq = q[i]
    zekken.append(qq)
    num.append((qq, pp))

num.sort()
result = []
for pp, qq in num:
    result.append(zekken[qq - 1])

print(" ".join(map(str, result)))
