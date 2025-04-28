n = int(input())

count = 0
login = False
for _ in range(n):
    s = input()

    if s == "login":
        login = True
    elif s == "logout":
        login = False
    elif s == "private":
        if not login:
            count += 1
    else:
        continue

print(count)
