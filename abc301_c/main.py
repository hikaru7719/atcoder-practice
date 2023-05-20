s = list(input())
t = list(input())

atcoder = "atcoder"
alpha = list('abcdefghijklmnopqrstuvwxyz')

smap = {}
tmap = {}

for aa in alpha:
    smap[aa] = 0
    tmap[aa] = 0

s_count = 0
t_count = 0

for i in range(0, len(s)):
    c = s[i]
    if c == '@':
        s_count +=1
        continue
    smap[c] = smap[c] + 1

for i in range(0, len(s)):
    c = t[i]
    if c == '@':
        t_count +=1
        continue
    tmap[c] = tmap[c] + 1

bad = False
for aa in alpha:
    ss = smap[aa]
    tt = tmap[aa]
    if ss == tt:
        continue
    elif aa in atcoder:
        if ss < tt:
            s_count = s_count - (tt - ss)
        else:
            t_count = t_count - (ss - tt)
    else:
        bad = True
        break 

if s_count < 0 or t_count < 0:
    bad = True
if s_count != t_count:
    bad = True

if bad == False:
    print("Yes")
else:
    print("No")
