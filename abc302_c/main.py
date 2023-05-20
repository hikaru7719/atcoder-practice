import itertools

n, m = map(int, input().split())
l = []
for _ in range(0, n):
    l.append(input())

for perm in itertools.permutations(l):
    flag = True
    for i in range(n-1):
        count = 0
        for j in range(m):
            if perm[i][j] != perm[i+1][j]:
                count += 1
        if count != 1:
            flag = False
    if flag:
        print("Yes")
        exit()

print("No")
