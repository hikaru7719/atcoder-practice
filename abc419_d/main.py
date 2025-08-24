n, m = map(int, input().split())
s = input()
t = input()

cum = [0] * (n + 1)
for _ in range(m):
    l, r = map(int, input().split())
    cum[l - 1] += 1
    cum[r] -= 1

for i in range(n):
    cum[i + 1] += cum[i]
ans = [s[i] if cum[i] % 2 == 0 else t[i] for i in range(n)]
print("".join(ans))
