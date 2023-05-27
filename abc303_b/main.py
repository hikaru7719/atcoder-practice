from itertools import combinations

n, m = map(int, input().split())
l = []
for i in range(m):
    l.append(list(map(int, input().split())))

base = list(combinations([i for i in range(1,n+1)],2))

result = {}
for b in base:
    result[b] = False


for i in range(m):
    for j in range(n-1):
        a = l[i][j] 
        b = l[i][j+1]
        if a < b:
            result[(a,b)] = True
        else:
            result[(b,a)] = True

count = 0
for r in result.values():
    if r == False:
        count +=1

print(count)
