t = int(input())

for _ in range(t):
    n = int(input())
    s = list("s" + input())
    ok = [0] * (1 << n)
    ok[0] = 1
    for i in range(1 << n):
        if ok[i] == 0:
            continue
        for j in range(n):
            if i & (1 << j):
                continue
            next = i | (1 << j)
            if s[next] == "0":
                ok[next] = 1
    if ok[-1]:
        print("Yes")
    else:
        print("No")
