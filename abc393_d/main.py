n = int(input())
s = list(input())

count = 0
left_index = None
left_count = 0
right_index = None
right_count = 0
i = 0
j = len(s) - 1
if s[i] == "1":
    left_count += 1
    left_index = i
if s[j] == "1":
    right_count += 1
    right_index = j

while not (j - 1 < i + 1):
    i += 1
    j -= 1
    if j == i:
        if s[i] == "0":
            count += min(left_count, right_count)
    if s[i] == "1":
        left_count += 1
        left_index = i
    if s[i] == "0":
        count += left_count
    if s[j] == "1":
        right_count += 1
        right_index = j
    if s[j] == "0":
        count += right_count

if not left_index or not right_index:
    print(0)
else:
    print(count)
