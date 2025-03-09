n, m = map(int, input().split())

g: list[list[int]] = [[] for _ in range(n)]

for _ in range(m):
    v1, v2 = map(int, input().split())
    v1 -= 1
    v2 -= 1
    g[v1].append(v2)
    g[v2].append(v1)

visited = [False] * n


def dfs(current: int) -> None:
    for next in g[current]:
        if visited[next]:
            continue
        visited[next] = True
        dfs(next)


result = 0
for i in range(n):
    if visited[i]:
        continue
    result += 1
    visited[i] = True
    dfs(i)

print(result)
