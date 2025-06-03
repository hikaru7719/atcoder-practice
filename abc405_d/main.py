from collections import deque

h, w = map(int, input().split())
maze = [input().strip() for _ in range(h)]
dist = [["" for _ in range(w)] for _ in range(h)]

start = []
for i in range(h):
    for j in range(w):
        if maze[i][j] == "#":
            dist[i][j] = "#"
        if maze[i][j] == "E":
            dist[i][j] = "E"
            start.append((i, j))

dx = [-1, 1, 0, 0]
dy = [0, 0, -1, 1]

queue: deque[tuple[int, int]] = deque(start)

done_char = [
    "^",  # 上矢印が書かれた通路マス
    "v",  # 下矢印が書かれた通路マス
    "<",  # 左矢印が書かれた通路マス
    ">",  # 右矢印が書かれた通路マス
    "#",
    "E",
]


def decide_arrow(nx, ny):
    t = (nx, ny)
    if t == (-1, 0):
        return "v"
    if t == (1, 0):
        return "^"
    if t == (0, -1):
        return ">"
    if t == (0, 1):
        return "<"


while len(queue) > 0:
    x, y = queue.popleft()
    for i in range(4):
        nx, ny = x + dx[i], y + dy[i]
        if 0 <= nx and nx < h and 0 <= ny and ny < w and dist[nx][ny] == "":
            dist[nx][ny] = decide_arrow(dx[i], dy[i])
            queue.append((nx, ny))

for i in range(h):
    print("".join(dist[i]))
