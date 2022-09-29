def test():
    x, y, a, b = [int(_) for _ in input().split()]
    if x > y:
        x, y = y, x
    if a > b:
        a, b = b, a
    l, r = 0, 1000000001
    while l < r:
        flag = False
        m = (l + r + 1) // 2
        ux, uy = x - m * a, y - m * a
        if ux >= 0 and uy >= 0:
            if a == b:
                flag = True
            elif (ux // (b - a)) + (uy // (b - a)) >= m:
                flag = True

        if flag == True:
            l = m
        else:
            r = m - 1

    print(l)


for tc in range(int(input())):
    test()