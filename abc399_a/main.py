n = int(input())
s = list(input())
t = list(input())

count = 0
for i in range(n):
    si = s[i]
    ti = t[i]
    if si != ti:
        count += 1

print(count)
