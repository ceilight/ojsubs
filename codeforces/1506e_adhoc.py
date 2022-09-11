import io, os, sys

pypyin = io.BytesIO(os.read(0, os.fstat(0).st_size)).readline
cpyin = sys.stdin.readline
input = pypyin if 'PyPy' in sys.version else cpyin

def strput():
    return input().decode() if 'PyPy' in sys.version else input()

# code starts here

from collections import deque


def test():
    n = int(input())
    q = [int(x) for x in input().split()]

    mn, mx = [], []
    to_mn, to_mx = deque(), deque()
    last = 0
    for i in range(n):
        if q[i] != last:
            mn.append(q[i])
            mx.append(q[i])
            val = last + 1
            while val < q[i]:
                to_mn.append(val)
                to_mx.append(val)
                val += 1
            last = q[i]
        else:
            mn.append(to_mn.popleft())
            mx.append(to_mx.pop())
    print(*mn)
    print(*mx)


def main():
    tc = int(input())
    for t in range(tc):
        test()


if __name__ == "__main__":
    main()