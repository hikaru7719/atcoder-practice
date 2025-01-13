import bisect

n = int(input())
s = list(map(int, input().split()))
ss = sorted(s)

result = 0
for i in reversed(range(n)):
    large = ss[i]
    i = large / 2
    index = bisect.bisect(ss, i)
    result += index
print(result)
