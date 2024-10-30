import sys

n, m = map(int, input().split())
l = []
for _ in range(m):
    a,b = map(int ,input().split())
    l.append((a,b))

r1 = {}
r2 = {}
for (a,b) in l:
    r1[b] = True
    r2[a] = True
    r2[b] = True

if len(r2) != n:
    print(-1)
    sys.exit()

r = []
for i in range(1,n+1):
    if i in r1:
        continue
    r.append(i)

if 0 < len(r):
    if 1 < len(r):
        print(-1)
    else:
        print(r[0])
else:
    print(-1)
