import sys

def ask(t, i, j, x):
    print(f'? {t} {i} {j} {x}')
    sys.stdout.flush()
    x = int(input())
    return x

def test():
    n = int(input())
    i = 1
    idx = n
    a = [0] * n
    while(i + 1 <= n):
        ans = ask(2, i, i + 1, 1)
        if ans == 1:
            idx = i
            break
        elif ans == 2:
            ans = ask(2, i + 1, i, 1)
            if ans == 1:
                idx = i + 1
                break
        i += 2

    a[idx - 1] = 1
    for i in range(1, n + 1):
        if i == idx:
            continue
        ans = ask(1, idx, i, n - 1)
        a[i - 1] = ans

    print("! "+" ".join(map(str, a)))


tc = int(input())
for t in range(tc):
    test()