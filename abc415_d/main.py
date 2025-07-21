n, m = map(int, input().split())

ll = []
for _ in range(m):
    a, b = map(int, input().split())
    c = a - b
    ll.append((c, (a, b)))

ll.sort(key=lambda x: x[0])

current = n
ans = 0

for c, (a, b) in ll:
    if a <= current:
        d = (current - a) // c + 1
        current -= d * c
        ans += d

print(ans)
