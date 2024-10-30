import sys
n = int(input())
l = list(map(int, input().split()))
l1 = l[0]
if len(l) == 1:
    print(0)
    sys.exit()

lo = l[1:] 

if l1 < max(lo):
    print(max(lo) - l1 + 1)
elif l1 == max(lo):
    print(1)
else:
    print(0)