n = int(input())
p = list(map(int, input().split()))
result = [0 for _ in range(n)]

temp: list[tuple[int, int]] = []

for i in range(n):
    temp.append((i, p[i]))
temp.sort(key=lambda x: x[1], reverse=True)

rank = 1
same = 0
for ii, (i, value) in enumerate(temp):
    result[i] = rank
    if ii < n - 1:
        if temp[ii + 1][1] == value:
            same += 1
        else:
            rank += 1 + same
            same = 0

print("\n".join(map(str, result)))
