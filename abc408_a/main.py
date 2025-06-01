n, s = map(int, input().split())
a = list(map(int, input().split()))

last = 0
for i, aa in enumerate(a):
    if i == 0:
        if aa <= s:
            last = aa
            continue
        else:
            print("No")
            exit()
    else:
        if aa - last <= s:
            last = aa
            continue
        else:
            print("No")
            exit()
print("Yes")
