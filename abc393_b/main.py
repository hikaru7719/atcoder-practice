s = list(input())

count = 0
for i in range(1, 50):
    for j in range(len(s)):
        a = j
        b = j + i
        c = j + 2 * i
        if c < len(s):
            if s[a] == "A" and s[b] == "B" and s[c] == "C":
                count += 1

print(count)
