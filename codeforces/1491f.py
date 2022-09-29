import sys

def ask(l, r, ls, rs):
    print(f'? {l} {r}')
    print(" ".join(map(str, range(ls, ls + l))))
    print(" ".join(map(str, range(rs, rs + r))))
    sys.stdout.flush()
    x = int(input())
    return x

def test():
    n = int(input())
    idx = 0
    ans = []
    for i in range(2, n + 1):
        if ask(i - 1, 1, 1, i) != 0:
            idx = i
            break

    for i in range(i + 1, n + 1):
        if ask(1, 1, idx, i) == 0:
            ans.append(i)
    
    left = 1
    right = idx - 1
    while(left < right):
        mid = (left + right) >> 1
        if ask(mid, 1, 1, idx) == 0:
            left = mid + 1
        else:
            right = mid

    for i in range(1, idx):
        if i != left:
            ans.append(i)

    print(f'! {len(ans)} '+" ".join(map(str, ans)))


tc = int(input())
for t in range(tc):
    test()