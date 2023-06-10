n = int(input())
l = list(map(int, input().split()))

q = int(input())

ql = []
for _ in range(q):
    a,b = map(int, input().split())
    ql.append((a,b))


s = []

a = 1
current = 0
for i in range(l[n-1] + 1):
    s.append(current)
    if a % 2 == 1:
        if i == l[a] - 1:
            a = a + 1
    else:
        if i == l[a]:
            a = a + 1
        else:
            current = current + 1

for (a,b)in ql:
    print(s[b] - s[a])
