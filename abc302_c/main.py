import itertools

n, m = map(int, input().split())
l = []
for _ in range(0, n):
    l.append(input())

flag = False
for perm in itertools.permutations(l):
    for i in range(0, len(perm)-1):
        count = 0
        for j in range(0, m):
            if perm[i][j] != perm[i+1][j]:
                count = count + 1
        if count == 1:
            flag = True

if flag == True:
    print("Yes")
else:
    print("No")
