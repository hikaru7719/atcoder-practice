p, q = input().split()
m = {'A':0, 'B':3, 'C': 4,'D':8,'E':9,'F':14,'G':23,}

a = m[p]
b = m[q]

if a < b:
    print(b-a)
else:
    print(a-b)
