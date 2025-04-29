n, m = map(int, input().split())
vec = [0] * n
for i in range(m):
    a, b = map(int, input().split())
    vec[(a + b) % n] += 1

mm = m * (m - 1) / 2

for count in vec:
    mm -= count * (count - 1) / 2

print(int(mm))
