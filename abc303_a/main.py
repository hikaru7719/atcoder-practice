n = int(input())
s = input()
t = input()

for i in range(n):
    ss = s[i]
    tt = t[i]
    if ss == tt:
        continue
    elif (ss == '1' and tt == 'l') or (ss == 'l' and tt == '1'):
        continue
    elif (ss == '0' and tt == 'o') or (ss == 'o' and tt == '0'):
        continue
    else:
        print("No")
        exit()

print("Yes")