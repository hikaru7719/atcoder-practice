n = int(input())
s = list(map(int, input().split()))

flag = True
for i in range(1, n):
    if s[i - 1] < s[i]:
        continue
    else:
        flag = False
        break

if flag:
    print("Yes")
else:
    print("No")
