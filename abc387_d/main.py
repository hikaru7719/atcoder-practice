from collections import deque

h, w = map(int, input().split())
sl = []
start = (0, 0)
goal = (0, 0)

for hh in range(h):
    s = input()
    if (idx := s.find("S")) != -1:
        start = (hh, idx)
    if (idx := s.find("G")) != -1:
        goal = (hh, idx)
    sl.append(list(s))

ans = 10**9
moves = [[(0, 1), (0, -1)], [(1, 0), (-1, 0)]]

for p in range(0, 2):
    d = [[10**9] * w for _ in range(h)]

    d[start[0]][start[1]] = 0
    q: deque[tuple[int, int]] = deque()
    q.append(start)
    while 0 < len(q):
        oh, ow = q.popleft()
        for dh, dw in moves[(oh + ow + p) & 1]:
            nh = oh + dh
            nw = ow + dw
            if nh < 0 or h <= nh or nw < 0 or w <= nw:
                continue
            if sl[nh][nw] == "#":
                continue
            if d[nh][nw] == 10**9:
                d[nh][nw] = d[oh][ow] + 1
                q.append((nh, nw))

    ans = min(ans, d[goal[0]][goal[1]])

if ans == 10**9:
    print(-1)
else:
    print(ans)
