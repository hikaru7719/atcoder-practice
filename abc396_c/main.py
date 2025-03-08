n, m = map(int, input().split())
b_index = 0
w_index = 0
b = list(map(int, input().split()))
w = list(map(int, input().split()))

result = 0

sum = 0

b.sort(reverse=True)
w.sort(reverse=True)


for _ in range(n + m):
    b_value = b[b_index] if b_index < n else -(10**9) - 1
    w_value = w[w_index] if w_index < m else -(10**9) - 1

    if w_index < b_index:
        if b_value < w_value:
            sum += w_value
            result = max(result, sum)
            w_index += 1
        else:
            sum += b_value
            result = max(result, sum)
            b_index += 1
    else:
        sum += b_value
        result = max(result, sum)
        b_index += 1


print(result)
