#include <bits/stdc++.h>

using namespace std;
using ll = long long;

ll C(int k, int n) {
    if(k > n) return 0ll;
    if(k * 2 > n) k = n - k;
    if(k == 0) return 1ll;
    ll res = n;
    for (ll i = 2; i <= 1ll * k; i++) {
        res *= (n - i + 1);
        res /= i;
    }
    return res;
}

void up() {
    int n; cin >> n;
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
        while(l < i && *st.rbegin() - *st.begin() > 2) {
            st.erase(st.begin());
            l++;
        }
        int len = i - l;
        ans += C(2, len);
    }
    cout << ans << "\n";
}

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    int tc; cin >> tc;
    while(tc--) up();
    return 0; 
}