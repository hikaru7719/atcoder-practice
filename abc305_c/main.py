h, w= map(int, input().split())
s = []

s.append(['x' for _ in range(w+2)])

for _ in range(h):
    x = "x" + input() + "x"
    s.append(list(x))

s.append(['x' for _ in range(w+2)])

for hh in range(1,h+1):
    for ww in range(1, w+1):
        if s[hh][ww] == ".":
            if s[hh+1][ww] == "#" and s[hh+1][ww+1] == "#" and s[hh][ww+1] == "#":
                print(hh,ww)
                exit()
            elif s[hh+1][ww] == "#" and s[hh+1][ww-1] == "#" and s[hh][ww-1] == "#":
                print(hh,ww)
                exit()
            elif s[hh-1][ww] == "#" and s[hh-1][ww+1] == "#" and s[hh][ww+1] == "#":
                print(hh,ww)
                exit()
            elif s[hh-1][ww] == "#" and s[hh-1][ww-1] == "#" and s[hh][ww-1] == "#":
                print(hh,ww)
                exit()
            else:
                continue
