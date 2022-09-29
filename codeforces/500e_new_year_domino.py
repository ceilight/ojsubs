import io, os, sys

if 'PyPy' in sys.version:
    input = io.BytesIO(os.read(0, os.fstat(0).st_size)).readline
    def strput():
        return input().decode()
else:
    input = sys.stdin.readline
    def strput():
        return input()

# code starts here

class Fenwick:
    size = 0
    bit = []
 
    def __init__(self, n):
        for i in range(n + 1):
            self.bit.append(0)
        self.size = n
 
    def update(self, pos, val):
        u = pos
        while u <= self.size:
            self.bit[u] += val
            u += u & (-u)
 
    def get(self, pos):
        res, u = 0, pos 
        while u > 0:
            res += self.bit[u]
            u -= u & (-u)
        return res
 
 
def main():
    n = int(input())
    pos = [-1]
    hgt = [-1]
    for i in range(1, n + 1):
        p, h = [int(x) for x in input().split()]
        pos.append(p)
        hgt.append(h)
 
    q = int(input())
    ans = [0 for x in range(q)]
    query = [[] for x in range(n + 1)]
    for i in range(1, q + 1):
        l, r = [int(x) for x in input().split()]
        query[l].append((r, i))
    
    stack = []
    tree = Fenwick(n)
    for i in range(n, 0, -1):
        while len(stack) > 0:
            top = stack[-1]
            if pos[top] > pos[i] + hgt[i]:
                need = pos[top] - pos[i] - hgt[i]
                tree.update(top, min(0, need - tree.get(top) + tree.get(top - 1)))
                break
            tree.update(top, - tree.get(top) + tree.get(top - 1))
            stack.pop()
 
        stack.append(i)
        for j in query[i]:
            ans[j[1] - 1] = tree.get(j[0])
        tree.update(i, pos[i] - pos[i - 1])
        
    print('\n'.join(map(str, ans)))


if __name__ == "__main__":
    main()