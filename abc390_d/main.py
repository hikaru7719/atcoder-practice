n = int(input())
a = list(map(int, input().split()))
s: set[int] = set()
b = [0] * n


def bfs(i):
    if i == n:
        x = 0
        for bb in b:
            x ^= bb
        s.add(x)
        return

    for j in range(i + 1):
        if j != i and b[j] == 0:
            continue
        b[j] += a[i]
        bfs(i + 1)
        b[j] -= a[i]


bfs(0)
print(len(s))
