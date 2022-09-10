// https://oj.vnoi.info/problem/joisc13_mascots

#include <bits/stdc++.h>

using namespace std;

const long long mod = 1e9 + 7;

int R, C, N;
long long f[3007][3007], fact[9000003], inv_fact[9000003], res;

long long modexp(long long a, long long b) {
    long long ans = 1;
    while(b > 0) {
        if(b % 2) ans = ans * a % mod;
        a = a * a % mod, b /= 2;
    }
    return ans;
}

long long F(int x, int y) {
    if(x > R || y > C) return 0;
    if(x == R && y == C) return 1;
    if(f[x][y]) return f[x][y];

    long long fw = F(x + 1, y) * fact[y] % mod;
    long long fh = F(x, y + 1) * fact[x] % mod;
    return f[x][y]= (fw + fh) % mod;
}

void set_fact(int n) {
    fact[0] = 1;
    for(int i = 1; i <= n; i++)
        fact[i] = fact[i - 1] * i % mod;

    inv_fact[n] = modexp(fact[n], mod - 2);
    for(int i = n; i >= 1; i--)
        inv_fact[i - 1] = inv_fact[i] * i % mod;
}

long long nCr(int n, int r) {
    if(r > n) return 0;
    return fact[n] * inv_fact[n - r] % mod * inv_fact[r] % mod;
}

int main() {
    scanf("%d%d%d", &R, &C, &N);

    set_fact(R * C);

    int x1 = R + 1, y1 = C + 1, x2 = 0, y2 = 0;
    
    for(int i = 0; i < N; i++) {
        int x, y;
        scanf("%d%d", &x, &y);
        x1 = min(x1, x), y1 = min(y1, y);
        x2 = max(x2, x), y2 = max(y2, y);
    }

    res = F(x2 - x1 + 1, y2 - y1 + 1) % mod;
    res = res * fact[(x2 - x1 + 1) *(y2 - y1 + 1) - N] % mod;
    res = res * nCr(R - (x2 - x1 + 1), x1 - 1) % mod;
    res = res * nCr(C - (y2 - y1 + 1), y1 - 1) % mod;
    
    printf("%lld\n", res);
    
    return 0;
}
