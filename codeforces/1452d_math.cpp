#include <bits/stdc++.h>

using namespace std;
using ll = long long;

const ll MOD = 998244353;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    int n; cin >> n;
    ll f[n]; f[0] = 1, f[1] = 1;
    for(int i = 2; i < n; i++) {
        f[i] = (f[i - 1] + f[i - 2]) % MOD;
    }
    ll res = f[n - 1];
    for(int i = 0; i < n; i++) {
        res = (res * ((1ll + MOD) / 2ll) % MOD) % MOD;
    }
    cout << res << "\n";
    return 0;
}