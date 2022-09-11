import io, os, sys

pypyin = io.BytesIO(os.read(0, os.fstat(0).st_size)).readline
cpyin = sys.stdin.readline
input = pypyin if 'PyPy' in sys.version else cpyin

def strput():
    return input().decode() if 'PyPy' in sys.version else input()

# code starts here

class UnionFind:
    size = 0
    par, ext = [], []
 
    def __init__(self, n):
        for i in range(n):
            self.par.append(i)
            self.ext.append(1)
        self.size = n
 
    def find(self, u):
        while u != self.par[u]:
            u = self.par[u]
        return u
 
    def unite(self, u, v):
        u, v = self.find(u), self.find(v)
        if u == v:
            return False
        if self.ext[u] < self.ext[v]:
            u, v = v, u
        self.ext[u] += self.ext[v]
        self.par[v] = u
        self.size -= 1
        return True
 

def main():
    n = int(input())
    g = [[] for x in range(n + 1)]
    for i in range(n - 1):
        u, v = [int(x) for x in input().split()]
        g[u].append(v)
        g[v].append(u)

    to_destroy = []
    dsu = UnionFind(n + 1)
    for u in range(1, n + 1):
        for v in g[u]:
            if v < u: continue
            if not dsu.unite(u, v):
                to_destroy.append((u, v))

    print(len(to_destroy))
    while to_destroy:
        cur = to_destroy.pop()
        for i in range(1, n):
            if dsu.unite(i, i + 1):
                cur += (i, i + 1)
                break
        print(*cur)


if __name__ == "__main__":
    main()