# https://oj.vnoi.info/problem/icpc21_mn_e

a, h = map(float, input().split())

left = 5*h*a/6
if a >= 100:
    right = 500*h/6
    mid = 0
else:
    right = 200*h/3
    x = h/2 - (h/12)*(2*a-100)/(100-a)
    if x < h/6 or x > h/3:
        mid = 0
    else:
        mid = (2*a-100+(6*(100-a)/h)*x)*(h-x)
print(max(left, right, mid))
