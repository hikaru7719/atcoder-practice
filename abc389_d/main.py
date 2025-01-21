r = int(input())


def inside(x, y, r):
    return (2 * x + 1) * (2 * x + 1) + (2 * y + 1) * (2 * y + 1) <= 4 * r * r


cnt = 0
up = r - 1
res = (r - 1) * 4 + 1

x = 1
while inside(x, 1, r):
    while not inside(x, up, r):
        up -= 1
    cnt += up
    x += 1

res += cnt * 4
print(res)
