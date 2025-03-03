n, q = map(int, input().split())
pigeon2back = [i for i in range(n)]  # 鳩とバックの対応
back2hall = [i for i in range(n)]  # バックと巣の対応
hall2back = [i for i in range(n)]  # 巣とバックの対応

for i in range(q):
    op = list(map(int, input().split()))
    if op[0] == 1:
        a = op[1] - 1
        b = op[2] - 1
        pigeon2back[a] = hall2back[b]
    elif op[0] == 2:
        a = op[1] - 1
        b = op[2] - 1
        hall2back[a], hall2back[b] = hall2back[b], hall2back[a]
        back2hall[hall2back[a]] = a
        back2hall[hall2back[b]] = b
    else:
        a = op[1] - 1
        ans = back2hall[pigeon2back[a]]
        print(ans + 1)
