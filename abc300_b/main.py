h,w = map(int, input().split())

a = []
b = []

for hh in range(0,h):
    a.append(input())

for hh in range(0,h):
    b.append(input())

def shift(a):
    result = []
    for aa in a:
        tmp = aa[0:1]
        tmp2 = aa[1:]
        result.append(str(tmp2+tmp))
    return result

def shift_row(a):
    tmp = a[0]
    a.pop(0)
    a.append(tmp)
    return a

def match(a, b, h, w):
    for hh in range(0,h):
        for ww in range(0, w):
            if a[hh][ww] != b[hh][ww]:
                return False
    return True

result = False
aa = a
for ww in range(0,w):
    if match(aa, b, h, w):
        result = True
        break
    aa = shift(aa)
    for hh in range(0, h):
        if match(aa, b, h, w):
            result = True
            break
        else:
           aa = shift_row(aa)
    else:
        continue
    break

if result:
    print("Yes")
else:
    print("No")

    