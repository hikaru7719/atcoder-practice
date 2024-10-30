n, m = list(map(int, input().split()))


class Adder:
    def __init__(self):
        self._add = 0
        self.table = {}

    def add(self, i, j):
        if 0 < i and i < n + 1 and 0 < j and j < n + 1:
            if (i, j) in self.table:
                self._add += 1
            else:
                self.table[(i, j)] = True
        else:
            self._add += 1

    def get_add(self):
        return self._add


adder = Adder()

for _ in range(m):
    a, b = list(map(int, input().split()))
    adder.add(a, b)
    adder.add(a + 2, b + 1)
    adder.add(a + 1, b + 2)
    adder.add(a - 1, b + 2)
    adder.add(a - 2, b + 1)
    adder.add(a - 2, b - 1)
    adder.add(a - 1, b - 2)
    adder.add(a + 1, b - 2)
    adder.add(a + 2, b - 1)

print(n * n - 9 * m + adder.get_add())
