n, a, b = map(int, input().split())
s = input()

if b == 0:
    print(s[a:])
else:
    print(s[a:-b])
