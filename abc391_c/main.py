n, q = map(int, input().split())
su = [1] * n
point = {i: i for i in range(n)}
count = 0
for _ in range(q):
    query = list(map(int, input().split()))
    if query[0] == 1:
        p = query[1] - 1
        h = query[2] - 1

        previous_su_h = su[h]
        previous_su_p = su[point[p]]
        su[h] += 1
        su[point[p]] -= 1
        if previous_su_h == 1:
            count += 1
        if previous_su_p == 2:
            count -= 1
        point[p] = h
    else:
        print(count)
