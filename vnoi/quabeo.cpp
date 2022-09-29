// https://oj.vnoi.info/problem/vmquabeo

#include <iostream>
#include <algorithm>

using namespace std;
using llong = long long;

const int N = 1000100;

struct node {
    int min, max;
    node() { min = 0, max = N; }
} seg[N << 1];

int h[N];

void build(int n) {
    for (int i = 0; i < n; i++)
        seg[n+i].min = seg[n+i].max = h[i];
    for (int i = n-1; i > 0; i--) {
        seg[i].min = min(seg[i<<1].min, seg[i<<1|1].min);
        seg[i].max = max(seg[i<<1].max, seg[i<<1|1].max); 
    }
}

int getmin(int n, int l, int r) {
    int res = N;
    l += n, r += n;
    while (l < r) {
        if (l & 1) res = min(res, seg[l++].min);
        if (r & 1) res = min(res, seg[--r].min);
        l >>= 1, r >>= 1;
    }
    return res;
}

int getmax(int n, int l, int r) {
    int res = 0;
    l += n, r += n;
    while (l < r) {
        if (l & 1) res = max(res, seg[l++].max);
        if (r & 1) res = max(res, seg[--r].max);
        l >>= 1, r >>= 1;
    }
    return res;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, l, d;
    cin >> n >> l >> d;
    for (int i = 0; i < n; i++)
        cin >> h[i];
    build(n);
    int u = 0, v;
    int hmin = N, hmax = 0;
    llong cnt = 0;
    for (v = 0; v < n; v++) {
        hmin = min(hmin, h[v]);
        hmax = max(hmax, h[v]);
        while (hmax - hmin > d && u < v) {
            u++;
            hmin = getmin(n, u, v+1);
            hmax = getmax(n, u, v+1);
        }
        if (v - u + 1 > l)
            cnt += v - u + 1 - l;
    }
    cout << cnt << "\n";
    return 0;
}
