n = int(input())
s = list(map(int, input().split()))

set_s = set(s)
list_s = sorted(list(set_s))

print(len(list_s))
print(" ".join(map(str, list_s)))
