n,m,h,k = map(int,input().split())
s = list(input())

item = {}

for _ in range(m):
    x, y = map(int,input().split())
    item[(x,y)] = True

x = 0
y = 0

for ss in s:
    if ss == 'R':
        x = x + 1
    elif ss == 'L':
        x = x - 1
    elif ss == 'U':
        y = y + 1
    elif ss == 'D':
        y = y - 1
    else:
        pass
    h = h - 1

    if h < 0:
        print("No")
        exit()
    if (x,y) in item and item[(x,y)] == True and h < k:
        item[(x,y)] = False
        h = k


print("Yes")
