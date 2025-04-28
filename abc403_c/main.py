ii = list(map(int, input().split()))

n = ii[0]
m = ii[1]
q = ii[2]


all = [False] * n
map_result: list[dict[int, bool]] = [{} for i in range(n)]


for i in range(q):
    values = list(map(int, input().split()))

    if values[0] == 1:
        x = values[1] - 1
        y = values[2] - 1
        map_result[x][y] = True
    elif values[0] == 2:
        x = values[1] - 1
        all[x] = True
    elif values[0] == 3:
        x = values[1] - 1
        y = values[2] - 1
        mr = map_result[x]
        if y in mr:
            print("Yes")
        elif all[x]:
            print("Yes")
        else:
            print("No")
