h, w = map(int, input().split())

l = []

for _ in range(0, h):
    l.append(list(input()))

def check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
    if l[r1][c1] == 's' and l[r2][c2] == 'n' and l[r3][c3] == 'u'  and l[r4][c4] == 'k' and l[r5][c5] == 'e':
        print(r1+1,' ',c1+1)
        print(r2+1,' ',c2+1)
        print(r3+1,' ',c3+1)
        print(r4+1,' ',c4+1)
        print(r5+1,' ',c5+1)
        return True
    else:
        return False


for i in range(0, h):
    for j in range(0, w):
        if i + 4 < h:
            r1, r2, r3, r4, r5 = i, i+1, i+2, i+3, i+4
            c1, c2, c3, c4, c5 = j, j, j, j, j
            if check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
                break
        if 0 <= i - 4:
            r1, r2, r3, r4, r5 = i, i-1, i-2, i-3, i-4
            c1, c2, c3, c4, c5 = j, j, j, j, j
            if check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
                break
        if j + 4 < w:
            r1, r2, r3, r4, r5 = i, i, i, i, i
            c1, c2, c3, c4, c5 = j, j+1, j+2, j+3, j+4
            if check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
                break
        if 0 <= j - 4:
            r1, r2, r3, r4, r5 = i, i, i, i, i
            c1, c2, c3, c4, c5 = j, j-1, j-2, j-3, j-4
            if check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
                break
        if i + 4 < h and j + 4 < w:
            r1, r2, r3, r4, r5 = i, i+1, i+2, i+3, i+4
            c1, c2, c3, c4, c5 = j, j+1, j+2, j+3, j+4
            if check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
                break
        if i + 4 < h and 0 <= j - 4:
            r1, r2, r3, r4, r5 = i, i+1, i+2, i+3, i+4
            c1, c2, c3, c4, c5 = j, j-1, j-2, j-3, j-4
            if check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
                break
        if 0 <= i - 4 and j + 4 < w:
            r1, r2, r3, r4, r5 = i, i-1, i-2, i-3, i-4
            c1, c2, c3, c4, c5 = j, j+1, j+2, j+3, j+4
            if check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
                break
        if 0 <= i - 4 and 0 <= j - 4:
            r1, r2, r3, r4, r5 = i, i-1, i-2, i-3, i-4
            c1, c2, c3, c4, c5 = j, j-1, j-2, j-3, j-4
            if check(l, r1, c1, r2, c2, r3, c3, r4, c4, r5, c5):
                break
    else:
        continue
    break
