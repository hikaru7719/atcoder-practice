t = int(input())

for _ in range(t):
    n, m = map(int, input().split())
    a = list(map(int, input().split()))
    b = list(map(int, input().split()))

    a.sort(reverse=True)
    b.sort()

    ans = 0
    idx = 0
    c = 0
    for aa in a:
        while idx < n and b[idx] + aa < m:
            idx += 1
        if idx >= n:
            break
        c += 1
        idx += 1
    print(sum(a) + sum(b) - m * c)
