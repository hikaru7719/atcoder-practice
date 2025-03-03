n, q = map(int, input().split())
h = {i: (0, i) for i in range(n)}
index = 0
did_op_i = []
did_op = []

for i in range(q):
    op = list(map(int, input().split()))
    if op[0] == 1:
        a = op[1] - 1
        b = op[2] - 1
        h[a] = (i, b)
        did_op_i.append(index)
    elif op[0] == 2:
        a = op[1] - 1
        b = op[2] - 1
        did_op.append((a, b))
        did_op_i.append(index)
        index += 1
    else:
        a = op[1]
        last_i, s = h[a]
        last_op_i = did_op_i[last_i]
        latest_op_i = did_op_i[i - 1]

        for aa, bb in did_op[last_op_i : latest_op_i + 1]:
            if aa == s:
                s = aa
            elif bb == s:
                s = bb
        print("****")
        print(s + 1)
        print("****")

        h[a] = (i, s)
        did_op_i.append(index)
