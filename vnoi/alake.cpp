#include <bits/stdc++.h>

using namespace std;

const int INF = 1e9 + 7;
int pre[100003], suf[100003], w[100003], h[100003];

int main() {
    ios::sync_with_stdio(0); cin.tie(0);

    int n; cin >> n;

    h[0] = h[n + 1] = INF;

    for(int i = 1; i <= n; i++)
        pre[i] = i - 1, suf[i] = i + 1;
    
    for(int i = 1; i <= n; i++)
        cin >> w[i] >> h[i];

    long long tot = 0ll;
    long long res[n + 1];
    int cur = min_element(h + 1, h + 1 + n) - h;

    while(cur > 0 && cur <= n) {
        res[cur]= tot + 1ll * w[cur];

        suf[pre[cur]] = suf[cur];
        pre[suf[cur]] = pre[cur];

        if(h[pre[cur]] < h[suf[cur]]) {
            tot += 1ll* w[cur] * (h[pre[cur]] - h[cur]);
            w[pre[cur]] += w[cur];

            cur = pre[cur];
            while(cur > 0 && h[pre[cur]] < h[cur])
                cur = pre[cur];
        }
        else {
            tot += 1ll* w[cur] * (h[suf[cur]] - h[cur]);
            w[suf[cur]] += w[cur];

            cur = suf[cur];
            while(cur <= n && h[suf[cur]] < h[cur])
                cur = suf[cur];
        }
    }

    for(int i = 1; i <= n; i++) cout << res[i] << "\n";

    return 0;
}
