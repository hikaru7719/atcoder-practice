from collections import deque

h, w = map(int, input().split())
a = [input() for _ in range(h)]

sx, sy, gx, gy = -1, -1, -1, -1

# 移動回数
for k in range(h):
    for j in range(w):
        if a[k][j] == "S":
            sx, sy = k, j
        if a[k][j] == "G":
            gx, gy = k, j

INF = 10**9
d = [[[INF] * w for _ in range(h)] for _ in range(2)]
dx = [-1, 1, 0, 0]
dy = [0, 0, -1, 1]

# 現在地、スイッチの状態
queue: deque[tuple[int, int, int]] = deque()
queue.append((0, sx, sy))
d[0][sx][sy] = 0

while queue:
    c, x, y = queue.popleft()
    for k in range(4):
        xx, yy = x + dx[k], y + dy[k]
        if (
            not (0 <= xx < h and 0 <= yy < w)
            or a[xx][yy] == "#"
            or (c == 0 and a[xx][yy] == "x")
            or (c == 1 and a[xx][yy] == "o")
        ):
            continue

        cc = c ^ (a[xx][yy] == "?")
        if d[cc][xx][yy] != INF:
            continue
        d[cc][xx][yy] = d[c][x][y] + 1
        queue.append((cc, xx, yy))

ans = min(d[0][gx][gy], d[1][gx][gy])
print(-1 if ans == INF else ans)
