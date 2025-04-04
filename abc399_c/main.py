n, m = map(int, input().split())


class UnionFind:
    def __init__(self, n: int):
        self.parent = [-1] * n
        self.size = [1] * n

    def root(self, x: int) -> int:
        while True:
            if self.parent[x] == -1:
                return x
            x = self.parent[x]

    def union(self, x: int, y: int) -> None:
        root_x = self.root(x)
        root_y = self.root(y)
        if root_x == root_y:
            return
        if self.size[root_x] < self.size[root_y]:
            self.parent[root_x] = root_y
            self.size[root_y] += self.size[root_x]
        else:
            self.parent[root_y] = root_x
            self.size[root_x] += self.size[root_y]

    def same(self, x: int, y: int) -> bool:
        return self.root(x) == self.root(y)


k = n

uf = UnionFind(n)
for i in range(m):
    a, b = map(int, input().split())
    a -= 1
    b -= 1
    if not uf.same(a, b):
        uf.union(a, b)
        k -= 1

print(m - n + k)
