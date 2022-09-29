from math import ceil

def test():
    n = int(input())
    res = []
    while n > 2:
        rt = ceil(n ** (1 / 2))
        for i in range(rt + 1, n):
            res.append((i, i + 1))
        res.append((n, rt))
        res.append((n, rt))
        n = rt
    print(len(res))
    for sol in res:
        print(f'{sol[0]} {sol[1]}') 


tc = int(input())
for t in range(tc):
    test()