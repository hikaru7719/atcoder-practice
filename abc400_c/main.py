import math

n = int(input())

check = set()
count = 0


def calc_2_n(x) -> int:
    if x == 0:
        return 0
    if x == 1:
        return 1
    i = 0
    while True:
        if x < 2**i:
            break
        i += 1
    return i


# for i in range(1, 60):
for i in range(1, 9):

    base = 2**i
    length = math.floor(math.sqrt(n / base))
    # print(f"math.floor(math.sqrt({n} / 2**{i}))={length}")
    print(f"calc_2_n({length})={calc_2_n(length)}")
    print(f"count+={length - calc_2_n(length)}")
    count += length - calc_2_n(length)

    # k = 1
    # while True:
    #     target = (2**i) * (k**2)
    #     if target <= n:
    #         print(f"(2**{i}) * ({k}**2)={(2**i) * (k**2)}")
    #         if target not in check:
    #             count += 1
    #             check.add(target)
    #     else:
    #         break
    #     k += 1


print(f"calc_2_n({n})={calc_2_n(n)}")
count += calc_2_n(n) - 1
print(count)
