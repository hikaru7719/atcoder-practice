n = int(input())
s = list(map(int, input().split()))

max_count = 1
for i, ss in enumerate(s):
    for j in range(1, n):
        count = 0
        current = i
        while current < n:
            if s[current] == ss:
                count += 1
                current += j
                max_count = max(count, max_count)
            else:
                break

print(max_count)
