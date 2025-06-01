n, m = map(int, input().split())
a = list(map(int, input().split()))


def check(a, b):
    for i in b:
        if i not in a:
            return False
    return True


ll = [i for i in range(1, m + 1)]
count = 0
while True:
    if check(a, ll):
        a = a[:-1]
        count += 1
    else:
        print(count)
        break
