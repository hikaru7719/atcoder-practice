q, n, m = map(int, input().split())
s = list(input())

sub = s[n - 1 : m]

print(sub)
for ss in sub:
    if ss != "o":
        print("No")
        exit()

print("Yes")
