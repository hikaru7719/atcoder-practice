n = int(input())
s2 = input()
s = list(filter(lambda x: x != "", s2.split("-")))

if "-" not in s2 or "o" not in s2:
    print(-1)
else:
    max = 0
    for ss in s:
        if max < len(ss):
            max = len(ss)
    print(max)
