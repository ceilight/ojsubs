#include <bits/stdc++.h>

using namespace std;
using ll = long long;

const ll mod = 1e9 + 7;
ll fact[200007];

ll ext_gcd(ll a, ll b, ll &m, ll &n) {
    if(b == 0) {
        m = 1, n = 0;
        return a;
    }
    ll m1, n1;
    ll gcd = ext_gcd(b, a % b, m1, n1);
    m = n1, n = m1 - (a / b) * n1;
    return gcd;
}

ll mod_inv(ll x) {
    ll m, n;
    ext_gcd(x, mod, m, n);
    return (m % mod + mod) % mod;
}

ll C(ll k, ll n) {
    if(k > n) return 0;
    return (fact[n] * mod_inv(fact[n-k] * fact[k] % mod)) % mod;
}

void up() {
    ll n, m, k; cin >> n >> m >> k;
    int a[n];
    for(int i = 0; i < n; i++) {
        cin >> a[i];
    }
    sort(a, a + n);
    multiset<int> st;
    int l = 0;
    ll ans = 0;
    for(int i = 0; i < n; i++) {
        st.insert(a[i]);
        while(l < i && *st.rbegin() - *st.begin() > k) {
            st.erase(st.begin());
            l++;
        }
        ll len = i - l;
        ans = (ans + C(m - 1, len)) % mod;
    }
    cout << ans << "\n";
}

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    fact[0] = 1;
    for(int i = 1; i < 200007; i++) {
        fact[i] = (fact[i - 1] * i) % mod;
    }
    int tc; cin >> tc;
    while(tc--) up();
    return 0; 
}