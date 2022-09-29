import io
import os
import sys
import math


class SegmentTree:
    def __init__(self, list, fun):
        self.size = len(list)
        self.seg = [0] * (self.size<<1)
        self.fun = fun
        self.seg[self.size:] = list[::]
        for i in range(self.size-1, 0, -1):
            self.seg[i] = self.fun(self.seg[i<<1], self.seg[i<<1|1])
 
    def modify(self, u, v):
        u += self.size
        self.seg[u] = v
        while u > 1:
            self.seg[u>>1] = self.fun(self.seg[u], self.seg[u^1])
            u >>= 1
    
    def query(self, u, v):
        u += self.size
        v += self.size
        res = 0
        while u < v:
            if u & 1:
                res = self.fun(res, self.seg[u])
                u += 1
            if v & 1:
                v -= 1
                res = self.fun(res, self.seg[v])
            u >>= 1
            v >>= 1
        return res


def solve():
    n = int(input())
    a = list(map(int, input().split()))
    d = [abs(a[i] - a[i+1]) for i in range(n-1)]
    tree = SegmentTree(d, math.gcd)
    res = 0
    l = 0
    for r in range(n-1):
        if tree.query(l, r+1) > 1:
            res = max(res, r-l+1)
        else:
            while tree.query(l, r+1) == 1:
                l += 1
    print(res+1)


def main():
    for test in range(int(input())):
        solve()


pypyin = io.BytesIO(os.read(0, os.fstat(0).st_size)).readline
cpyin = sys.stdin.readline
input = pypyin if 'PyPy' in sys.version else cpyin

def strput():
    return input().decode() if 'PyPy' in sys.version else input()

if __name__ == "__main__":
    main()
