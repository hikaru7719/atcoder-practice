n = int(input())
an = list(map(int, input().split()))

for i in range(0, len(an)):
    if i == len(an)-1:
        print(an[i])
        break
    else:
        print(an[i], end=" ")
    if an[i] < an[i+1]:
        for pad in range(an[i]+1, an[i+1]):
            print(pad, end=" ")
    else:
        for pad in range(an[i] -1 , an[i+1], -1):
            print(pad, end=" ")    