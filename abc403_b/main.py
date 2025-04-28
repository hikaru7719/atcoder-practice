t = input()
u = input()
u0 = u[0]

for i, tt in enumerate(t):
    if tt == u0 or tt == "?":
        match = True
        substr = t[i : i + len(u)]
        if len(substr) != len(u):
            continue
        for j, uu in enumerate(u):
            ss = substr[j]
            if ss == uu or ss == "?":
                continue
            else:
                match = False
                break
        if match:
            print("Yes")
            exit()
print("No")
