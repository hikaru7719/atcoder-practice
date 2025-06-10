h, w, n = map(int, input().split())
aa: list[list[int]] = [[] for _ in range(h + 1)]
bb: list[list[int]] = [[] for _ in range(w + 1)]
ux = [False] * (h + 1)
uy = [False] * (w + 1)
used = [False] * n
for i in range(n):
    (
        a,
        b,
    ) = map(int, input().split())
    aa[a].append(i)
    bb[b].append(i)


q = int(input())
for i in range(q):
    q, x = map(int, input().split())
    if q == 1:
        if ux[x]:
            print(0)
        else:
            sum = 0
            for j in aa[x]:
                if not used[j]:
                    sum += 1
                    used[j] = True
            print(sum)
            ux[x] = True
    elif q == 2:
        if uy[x]:
            print(0)
        else:
            sum = 0
            for j in bb[x]:
                if not used[j]:
                    sum += 1
                    used[j] = True
            print(sum)
            uy[x] = True
