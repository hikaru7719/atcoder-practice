q = int(input())

cards = [0 for i in range(100)]
print(len(cards))

for _ in range(q):

    query = list(map(int, input().split()))

    if query[0] == 1:
        cards.append(query[1])
    elif query[0] == 2:
        print(cards.pop(len(cards) - 1))
