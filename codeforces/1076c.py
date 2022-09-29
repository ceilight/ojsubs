def test():
    n = int(input())
    delta = n ** 2 - 4 * n
    if delta < 0:
        print("N")
    else:
        x = (n + delta ** (1 / 2)) / 2
        print(f'Y {x} {n - x}')


tc = int(input())
for t in range(tc):
    test()  