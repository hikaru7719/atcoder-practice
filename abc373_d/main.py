from collections import deque
from typing import Optional

n, m = map(int, input().split())
g: list[list[tuple[int, int]]] = [list() for _ in range(n)]
for _ in range(m):
    v1, v2, w = list(map(int, input().split()))
    g[v1 - 1].append((v2 - 1, w))
    g[v2 - 1].append((v1 - 1, -w))

ans: list[Optional[int]] = [None for _ in range(n)]
q: deque[int] = deque()

for i in range(n):
    if ans[i] is None:
        ans[i] = 0
        q.append(i)
        while 0 < len(q):
            cur = q.popleft()
            for target, w in g[cur]:
                if ans[target] is None:
                    ans[target] = ans[cur] + w
                    q.append(target)

print(" ".join(list(map(str, ans))))
