n, d = map(int, input().split())
a = list(map(int, input().split()))
m = 10**6 + 1
cnt = [0] * m

for x in a:
    cnt[x] += 1

if d == 0:
    print(sum(max(x - 1, 0) for x in cnt))
    exit()


def calc(x):
    if not x:
        return 0
    x = [0] + x
    dp = [0] * (len(x) + 1)
    for i in range(1, len(x)):
        dp[i + 1] = min(dp[i] + x[i], dp[i - 1] + x[i - 1])
    return dp[-1]


ans = 0
for i in range(d):
    ans += calc(cnt[i::d])

print(ans)
