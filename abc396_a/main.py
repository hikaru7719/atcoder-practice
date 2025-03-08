n = int(input())
s = list(map(int, input().split()))

result = False
for i in range(n - 2):
    if s[i] == s[i + 1] and s[i + 1] == s[i + 2]:
        result = True
        break

if result:
    print("Yes")
else:
    print("No")
