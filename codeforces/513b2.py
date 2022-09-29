n, m = [int(_) for _ in input().split()]
a = [0] * (n + 1)
l, r = 1, n

for i in range(1, n + 1):
    if m <= 1 << max((n - i - 1), 0):
        a[l] = i
        l += 1
    else:
        a[r] = i
        r -= 1
        m -= 1 << max((n - i - 1), 0)

a.pop(0)
print(" ".join(map(str, a)))