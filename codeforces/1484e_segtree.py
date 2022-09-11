import io, os, sys

pypyin = io.BytesIO(os.read(0, os.fstat(0).st_size)).readline
cpyin = sys.stdin.readline
input = pypyin if 'PyPy' in sys.version else cpyin

def strput():
    return input().decode() if 'PyPy' in sys.version else input()

# code starts here

inf = 10**9 + 7

class SegmentTree:
    def __init__(self, n):
        self.seg = [0] * (n * 2)
        self.size = n

    def modify(self, u, v):
        u += self.size
        self.seg[u] = v
        while u > 1:
            self.seg[u >> 1] = max(self.seg[u], self.seg[u ^ 1])
            u >>= 1
    
    def getmax(self, u, v):
        u += self.size
        v += self.size
        res = -inf
        while u < v:
            if u & 1:
                res = max(res, self.seg[u])
                u += 1
            if v & 1:
                v -= 1
                res = max(res, self.seg[v])
            u >>= 1
            v >>= 1
        return res


def main():
    n = int(input())
    height = [0] + [int(x) for x in input().split()]
    beauty = [0] + [int(x) for x in input().split()]

    tree = SegmentTree(n + 1)
    stack = []
    for i in range(1, n + 1):
        while stack and height[stack[-1]] > height[i]:
            stack.pop()
        if stack:
            lo = stack[-1]
            val = max(tree.getmax(lo, i) + beauty[i],
                      tree.getmax(lo, lo + 1))
        else:
            val = tree.getmax(0, i) + beauty[i]

        tree.modify(i, val)
        stack.append(i)

    print(tree.getmax(n, n + 1))


if __name__ == "__main__":
    main()