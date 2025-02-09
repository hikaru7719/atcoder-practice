h, w = map(int, input().split())
s = []

for i in range(h):
    s.append(list(input()))

a, b, c, d = None, None, None, None
for h, line in enumerate(s):
    for w, point in enumerate(line):
        if point == "#":
            if a is None:
                a = h
            else:
                a = min(a, h)
            if b is None:
                b = h
            else:
                b = max(b, h)
            if c is None:
                c = w
            else:
                c = min(c, w)
            if d is None:
                d = w
            else:
                d = max(d, w)


def check(s: list[list[str]], a: int, b: int, c: int, d: int) -> bool:
    for h in range(a, b + 1):
        for w in range(c, d + 1):
            if s[h][w] == ".":
                return False
    return True


if check(s, a, b, c, d):
    print("Yes")
else:
    print("No")
