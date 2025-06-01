from collections import deque

h, w = map(int, input().split())

maze = []
for _ in range(h):
    maze.append(list(input()))

a, b, c, d = map(int, input().split())

a -= 1
b -= 1
c -= 1
d -= 1

queue: deque[tuple[int, int]] = deque([])

dx = [-1, 1, 0, 0]
dy = [0, 0, -1, 1]

ans = [[h * w for _ in range(w)] for _ in range(h)]
ans[a][b] = 0
queue.append((a, b))

while 0 < len(queue):
    (x, y) = queue.popleft()
    if (x, y) == (c, d):
        print(ans[c][d])
        exit()

    for i in range(4):
        wall = False
        for j in [1, 2]:
            nx = x + dx[i] * j
            ny = y + dy[i] * j

            if nx < 0 or nx >= h or ny < 0 or ny >= w:
                continue
            if maze[nx][ny] == "#":
                wall = True

            if not wall:
                if j == 1:
                    if ans[nx][ny] > ans[x][y]:
                        ans[nx][ny] = ans[x][y]
                        queue.appendleft((nx, ny))
            else:
                if ans[nx][ny] > ans[x][y] + 1:
                    ans[nx][ny] = ans[x][y] + 1
                    queue.append((nx, ny))
