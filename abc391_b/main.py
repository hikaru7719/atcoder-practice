n, m = map(int, input().split())

S = []
for _ in range(n):
    S.append(list(input()))

T = []
for _ in range(m):
    T.append(list(input()))


def cut(s: list[list[str]], x: int, y: int) -> list[list[str]]:
    s_cut = []
    for i in range(x, x + m):
        s_cut.append(s[i][y : y + m])
    return s_cut


for i in range(n):
    for j in range(n):
        if i + m - 1 < n and j + m - 1 < n:
            s_cut = cut(S, i, j)
            if s_cut == T:
                print(i + 1, j + 1)
                exit()
