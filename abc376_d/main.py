from collections import deque
from typing import List

n, m = list(map(int, input().split()))
g: List[List[int]] = [list() for _ in range(n)]
inf = 10**9
d = [inf for _ in range(n)]
q: deque[int] = deque()

for _ in range(m):
    a, b = list(map(int, input().split()))
    g[a - 1].append(b - 1)

d[0] = 0
q.append(0)

while 0 < len(q):
    cur = q.popleft()
    for target in g[cur]:
        if target == 0:
            print(d[cur] + 1)
            exit()
        if d[target] == inf:
            d[target] = d[cur] + 1
            q.append(target)

print(-1)
