import bisect

s = input()
ss = list(s)
n = int(input())

scount = s.count('?')

l = []

for p in range(0, 2 ** scount):
    result = '0b'
    count = 0
    for s2 in ss:
        if s2 == '?':
            if 1 << count & p != 0:
                result += '1'
            else:
                result += '0'
            count +=1
        else:
            result += s2
    l.append(int(result, base=2))

l.sort()
index = bisect.bisect_left(l, n)
if index == 0:
    print(-1)
else:
    print(l[index-1])