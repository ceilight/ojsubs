// https://oj.vnoi.info/problem/atcoder_dp_x

#include <cstdio>
#include <algorithm>
#include <vector>

using namespace std;
using ll = long long;

struct Item {
    int w, s; ll v;
};

bool cmp(Item& a, Item& b) {
    return a.w+a.s < b.w+b.s;
}

int main() {
    int n;
    scanf("%d", &n);
    vector<Item> a(n);
    for (int i = 0; i < n; i++) {
        scanf("%d%d%lld", &a[i].w, &a[i].s, &a[i].v);
    }

    sort(a.begin(), a.end(), cmp);
    int wmx = -1, smx = -1; 
    for (Item x : a) {
        wmx = max(wmx, x.w);
        smx = max(smx, x.s);
    }

    vector<vector<ll>> f(n+1, vector<ll>(20100, 0));
    for (int i = 1; i <= n; i++) {
        for (int w = 0; w <= wmx+smx; w++) {
            f[i][w] = max(f[i][w], f[i-1][w]);
            if (w-a[i-1].w >= 0 && w-a[i-1].w <= a[i-1].s) {
                f[i][w] = max(f[i][w], f[i-1][w-a[i-1].w] + a[i-1].v);
            }
        }
    }
    ll ans = -1;
    for (int i = 0; i < 20100; i++) {
        ans = max(ans, f[n][i]);
    }
    printf("%lld\n", ans);

    return 0;
}
