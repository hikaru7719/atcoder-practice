x, y = map(int, input().split())


if (x + y) % 12 == 0:
    print(12)
else:
    print((x + y) % 12)
