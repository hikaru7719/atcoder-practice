q = int(input())

head = 0
last = 0
sub = 0
snake = []

for _ in range(q):
    query = list(map(int, input().split()))

    q0 = query[0]
    if q0 == 1:
        l = int(query[1])
        if len(snake) == 0:
            snake.append((0, l))
        else:
            (las_pos, las_l) = snake[last - 1]
            snake.append((las_pos + las_l, l))
        last += 1
    if q0 == 2:
        (_, l) = snake[head]
        head += 1
        sub += l
    if q0 == 3:
        k = int(query[1])
        (h, _) = snake[head + k - 1]
        print(h - sub)
