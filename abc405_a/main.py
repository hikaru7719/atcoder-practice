r, x = map(int, input().split())

if x == 1:
    if 1600 <= r and r <= 2999:
        print("Yes")
        exit()
else:
    if 1200 <= r and r <= 2399:
        print("Yes")
        exit()

print("No")
