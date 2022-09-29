#include <bits/stdc++.h>

using namespace std;
using ll = long long;

map<int, int> cnt;
map<pair<int, int>, int> cntp;
int n, w, h;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    cin >> n;
    for(int i = 0; i < n; i++) {
        cin >> w >> h;
        if(h > w) {
            swap(w, h);
        }
        cnt[w]++;
        if(w == h) continue;
        cnt[h]++;
        cntp[{w, h}]++;
    }
    ll ans = 0;
    for(auto w : cnt) {
        ans += 1ll * (w.second) * (w.second - 1) / 2ll;
    }
    for(auto s : cntp) {
        ans -= 1ll * (s.second) * (s.second - 1) / 2ll;
    }
    cout << ans << "\n";
    return 0;
}