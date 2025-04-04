n = int(input())


a = list(map(int, input().split()))
na = []
for i in range(n):
    na.append((a[i], i))

ll = sorted(na, key=lambda x: x[0], reverse=True)

max_num = None
last_value = None
for value, index in ll:
    if last_value == value:
        max_num = None
        continue
    if max_num is None:
        max_num = (value, index)
        last_value = value
    else:
        break

if max_num is None:
    print(-1)
else:
    print(max_num[1] + 1)
