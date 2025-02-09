N, W = map(int, input().split())
X = [0] * N
Y = [0] * N
blocks = [[] for _ in range(W + 1)]
for i in range(N):
    X[i], Y[i] = map(int, input().split())
    blocks[X[i]].append((Y[i], i))
cnt = [0] * N
disappear = [-1] * (N + 1)
for x in range(1, W + 1):
    blocks[x].sort(key=lambda p: p[0])
    for j, (y, i) in enumerate(blocks[x]):
        cnt[i] = j
        disappear[j] = max(disappear[j], y)
    disappear[len(blocks[x])] = 10**10
for i in range(N):
    disappear[i + 1] = max(disappear[i + 1], disappear[i] + 1)

Q = int(input())
for _ in range(Q):
    T, A = map(int, input().split())
    A -= 1
    print("Yes" if T < disappear[cnt[A]] else "No")
