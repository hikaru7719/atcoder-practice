s = list(input())

t = [""] * len(s)

for i, ss in enumerate(s):
    if ss == "#":
        t[i] = "#"

flag = True
for i, tt in enumerate(t):
    if tt == "#":
        flag = True
    if tt == "":
        if flag:
            t[i] = "o"
            flag = False
        else:
            t[i] = "."

print("".join(t))
