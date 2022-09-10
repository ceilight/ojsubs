import io
import os
import sys
import threading
from collections import deque


def bfs(x):
    q = deque()
    q.append(x)
    while q:
        u = q.popleft()
        for v in graph[u]:
            if dis[x][v] > dis[x][u]+1:
                dis[x][v] = dis[x][u]+1
                path[x][v] = path[x][u]
                q.append(v)
            elif dis[x][v] == dis[x][u]+1:
                path[x][v] += path[x][u]


def main():
    global graph, dis, path
    inf = 10**5

    n, m = list(map(int, input().split()))
    graph = [([]) for x in range(n+1)]
    dis = [([inf] * (n+1)) for row in range(n+1)]
    path = [([0] * (n+1)) for row in range(n+1)]
    for i in range(m):
        u, v = list(map(int, input().split()))
        graph[u].append(v)
        graph[v].append(u)
    for i in range(1, n+1):
        dis[i][i] = 0
        path[i][i] = 1
    bfs(1)
    splen, spnum = dis[1][n], path[1][n]
    safe = spnum
    for i in range(2, n):
        bfs(i)
        if dis[i][1]+dis[i][n] == splen:
            safe = max(safe, path[i][1]*path[i][n]*2)
    sys.stdout.write(str(round(safe/spnum, 10)))


fastin = io.BytesIO(os.read(0, os.fstat(0).st_size))
input = (fastin if 'PyPy' in sys.version else sys.stdin).readline
 
def strput():
    return input().decode() if 'PyPy' in sys.version else input()
 
if __name__ == "__main__":
    sys.setrecursionlimit(20200)
    threading.stack_size(1024000)
    thread = threading.Thread(target=main)
    thread.start()