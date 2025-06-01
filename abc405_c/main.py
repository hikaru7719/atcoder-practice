n = int(input())
a = list(map(int, input().split()))


sum = 0
for i in a:
    sum += i

result = 0
for i in a:
    sum -= i
    result += sum * i

print(result)
