n = int(input())
ai = list(map(int, input().split()))
bi = list(map(int, input().split()))

ai.sort(reverse=True)
bi.sort(reverse=True)

ans = None
for i in range(n):
    if i == n - 1 and len(bi) == n - 1:
        ans = ai[i]
        break
    if ai[i] <= bi[i]:
        continue
    elif ans is None:
        bi.insert(i, ai[i])
        ans = ai[i]
    else:
        ans = -1
        break

print(ans)
