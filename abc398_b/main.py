a = list(map(int, input().split()))

count = {}

for aa in a:
    if aa in count:
        count[aa] += 1
    else:
        count[aa] = 1

flag_a = 0
flag_b = 0
for key in count:
    if 2 < count[key]:
        flag_a += 1
        flag_b += 1
    elif 1 < count[key]:
        flag_b += 1

if 0 < flag_a and 1 < flag_b:
    print("Yes")
else:
    print("No")
