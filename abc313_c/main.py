import statistics, math

n = int(input())
a = list(map(int, input().split()))

z = statistics.mean(a)
x = math.floor(z)
y = math.ceil(z)

xPlus = 0
xMinus = 0
for aa in a:
    if aa < x:
        xPlus += x - aa
    else:
        xMinus += aa - x

yPlus = 0
yMinus = 0
for aa in a:
    if aa < y:
        yPlus += y - aa
    else:
        yMinus += aa - y

xx = 0
xz = 0
yy = 0
yz = 0
if xPlus < xMinus:
    xx = xPlus
    xz = xMinus - xPlus 
else:
    xx = xMinus
    xz = xPlus - xMinus


if yPlus < yMinus:
    yy = yPlus
    yz = yMinus - yPlus
else:
    yy = yMinus
    yz = yPlus - yMinus

if xz < yz:
    print(xx)
elif xz == yz:
    print(min(xx, yy))
else:
    print(yy)
