n = int(input())
a = list(list(input()) for _ in range(n))
ans: list[list[str]] = [["" for _ in range(n)] for __ in range(n)]

for i in range(n):
    for j in range(n):
        d = (min(i + 1, j + 1, n - i, n - j)) % 4
        ni = i
        nj = j
        for _ in range(d % 4):
            ni, nj = (
                nj,
                n - 1 - ni,
            )
        ans[ni][nj] = a[i][j]

for i in range(n):
    print("".join(ans[i]))
