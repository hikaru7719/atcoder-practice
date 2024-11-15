n, m = list(map(int, input().split()))
s = list(input())
count = 0
for i in range(n - 2):
    if s[i] + s[i + 1] + s[i + 2] == "ABC":
        count += 1

for _ in range(m):
    t, c = input().split()
    x = int(t) - 1
    if s[x] == "A" and x + 2 < n:
        if s[x] + s[x + 1] + s[x + 2] == "ABC":
            count -= 1
    elif s[x] == "B" and -1 < x - 1 and x + 1 < n:
        if s[x - 1] + s[x] + s[x + 1] == "ABC":
            count -= 1
    elif s[x] == "C" and -1 < x - 2:
        if s[x - 2] + s[x - 1] + s[x] == "ABC":
            count -= 1
    s[x] = c
    if s[x] == "A" and x + 2 < n:
        if s[x] + s[x + 1] + s[x + 2] == "ABC":
            count += 1
    elif s[x] == "B" and -1 < x - 1 and x + 1 < n:
        if s[x - 1] + s[x] + s[x + 1] == "ABC":
            count += 1
    elif s[x] == "C" and -1 < x - 2:
        if s[x - 2] + s[x - 1] + s[x] == "ABC":
            count += 1
    print(count)
