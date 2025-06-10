t = int(input())


def solve():
    n = int(input())
    s = list(input())

    for i in range(n - 1):
        if s[i] > s[i + 1]:
            target = s[i]
            result = s[:i]
            for j in range(i + 1, n):
                if s[j] <= target:
                    result.append(s[j])
                else:
                    result.append(target)
                    result.extend(s[j:])
                    print("".join(result))
                    return
            result.append(target)
            print("".join(result))
            return
    print("".join(s))


for _ in range(t):
    solve()
