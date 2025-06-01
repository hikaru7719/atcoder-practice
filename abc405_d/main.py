from collections import deque

h, w = map(int, input().split())

maze = []
start = []
for i in range(h):
    s = list(input())
    if "E" in s:
        j = s.index("E")
        start.append((i, j))
    maze.append(s)

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
        if 0 <= nx and nx < h and 0 <= ny and ny < w and maze[nx][ny] not in done_char:
            maze[nx][ny] = decide_arrow(dx[i], dy[i])
            queue.append((nx, ny))

for i in range(h):
    print("".join(maze[i]))
