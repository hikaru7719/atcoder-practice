n = int(input())
s = [[None for _ in range(n)] for _ in range(n)]


def rect(s: list, i: int, j: int, flag: bool) -> None:
    for k in range(i, j):
        for l in range(i, j):
            s[l][k] = flag
    return


for i in range(n):
    j = n - i
    if i <= j:
        if i % 2 == 0:
            rect(s, i, j, True)
        else:
            rect(s, i, j, False)

for i in range(n):
    for j in range(n):
        if s[i][j]:
            print("#", end="")
        else:
            print(".", end="")
    print()
