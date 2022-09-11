#include <iostream>
#include <algorithm>

using llong = long long;
using namespace std;

const int N = 300100;
const int M = 1000100;
int a[N];
int d[M];

struct node {
    llong tot, val;
} seg[N << 2];

void build(int i, int l, int r) {
    if(l == r) {
        seg[i].tot = seg[i].val = a[l];
        return;
    }
    int m = (l + r) >> 1;
    build(i << 1, l, m);
    build(i << 1 | 1, m + 1, r);
    seg[i].tot = seg[i << 1].tot + seg[i << 1 | 1].tot;
    seg[i].val = max(seg[i << 1].val, seg[i << 1 | 1].val);
}

void modify(int i, int l, int r, int u, int v) {
    if(l > r || l > v || r < u) return;
    if(seg[i].val <= 2) return;
    if(l == r) {
        seg[i].tot = seg[i].val = d[seg[i].tot];
        return;
    }
    int m = (l + r) >> 1;
    modify(i << 1, l, m, u, v);
    modify(i << 1 | 1, m + 1, r, u, v);
    seg[i].tot = seg[i << 1].tot + seg[i << 1 | 1].tot;
    seg[i].val = max(seg[i << 1].val, seg[i << 1 | 1].val);
}

llong get(int i, int l, int r, int u, int v) {
    if(l > r || l > v || r < u) return 0LL;
    if(u <= l && r <= v) return seg[i].tot;
    int m = (l + r) >> 1;
    return get(i << 1, l, m, u, v)
         + get(i << 1 | 1, m + 1, r, u, v);
}

void calc_divcnt() {
    for(int i = 1; i < M; i++)
        for(int j = i; j < M; j += i)
            ++d[j];
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int n, m;
    cin >> n >> m;
    for(int i = 1; i <= n; i++) cin >> a[i];
    build(1, 1, n);
    calc_divcnt();
    for(int i = 0; i < m; i++) {
        int t, u, v;
        cin >> t >> u >> v;
        if(t == 1)
            modify(1, 1, n, u, v);
        else
            cout << get(1, 1, n, u, v) << "\n";
    }
    return 0;
}