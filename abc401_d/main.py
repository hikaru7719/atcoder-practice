n, k = map(int, input().split())
t = list(input())

for i in range(n):
    if t[i] == "?":
        if (0 < i and t[i - 1] == "o") or (i < n - 1 and t[i + 1] == "o"):
            t[i] = "."

    if t[i] == "o":
        k -= 1

if k == 0:
    for i in range(n):
        if t[i] == "?":
            t[i] = "."


index = 0
segment = []
while index < n:
    if t[index] == "?":
        start = index
        length = 0
        while index < n and t[index] == "?":
            length += 1
            index += 1
        segment.append((start, length))
    else:
        index += 1

k_max = 0
for start, length in segment:
    if length % 2 == 0:
        k_max += length // 2
    else:
        k_max += (length + 1) // 2

if k_max == k:
    for start, length in segment:
        if length % 2 == 1:
            for j in range(length):
                t[start + j] = "o" if j % 2 == 0 else "."

print("".join(t))
