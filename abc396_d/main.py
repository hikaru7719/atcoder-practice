n, m = map(int, input().split())

graph: list[list[tuple[int, int]]] = [[] for _ in range(n)]

for _ in range(m):
    vv = list(map(int, input().split()))
    graph[vv[0] - 1].append((vv[1] - 1, vv[2]))
    graph[vv[1] - 1].append((vv[0] - 1, vv[2]))

result = 2**61


def dfs(current: int, xor_value: int, visited: list[bool]) -> None:
    for next, w in graph[current]:
        if next == n - 1:
            value = xor_value ^ w
            global result
            result = min(result, value)
            return
        if visited[next]:
            continue
        visited[next] = True
        dfs(next, xor_value ^ w, visited)
        visited[next] = False


visited = [False] * n
visited[0] = True
dfs(0, 0, visited)

print(result)
