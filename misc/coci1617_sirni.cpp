// https://oj.vnoi.info/problem/coci1617_r6_sirni

#include <cstdio>
#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;
using ll = long long;

const int N = 100100, P = 10000100;
int n, a[N];
int rnk[N], par[N];

int root(int u) {
    while (u != par[u]) u = par[u];
    return u;
}
 
bool unite(int u, int v) {
    u = root(u), v = root(v);
    if (u == v) return false;
    if (rnk[u] < rnk[v]) {
        par[u] = v;
    } else {
        par[v] = u;
        if (rnk[u] == rnk[v]) rnk[u]++;
    }
    return true;
}

struct Edge {
    int u, v;
};

// bool cmp(Edge a, Edge b) {
//     return a.w < b.w;
// }

vector<Edge> edges[P];
int nxtnode[P], pos[P];
bool vis[P];

int main() {
    scanf("%d", &n);
    for (int i = 1; i <= n; i++) {
        scanf("%d", &a[i]);
        nxtnode[a[i]] = a[i];
        if (pos[a[i]] == 0) {
            pos[a[i]] = i;
        }
    }
    int mx = *max_element(a+1, a+n+1);
    for (int i = mx+1; i >= 0; i--) {
        if (nxtnode[i] == 0) nxtnode[i] = nxtnode[i+1];
    }

    for (int i = 1; i <= n; i++) {
        int x = a[i];
        if (vis[x]) continue;
        vis[x] = true;
        
        for (int j = x+x; j <= mx; j += x) {
            if (j+x > mx || nxtnode[j] != nxtnode[j+x]) {
                int wt = nxtnode[j]-j;
                edges[wt].push_back({pos[nxtnode[j]], i});
            }
        }
        if (x+1 <= mx) {
            if (x+x > mx || nxtnode[x+1] != nxtnode[x+x]) {
                int wt = nxtnode[x+1]-x;
                edges[wt].push_back({pos[nxtnode[x+1]], i});
            }
        }
    }

    for (int i = 0; i < n; i++) par[i] = i, rnk[i] = 1;
    ll ans = 0;
    int cnt = 0;
    bool gud = false;
    for (int i = 0; i <= mx; i++) {
        for (Edge e: edges[i]) {
            if (unite(e.u, e.v)) {
                ans += i;
                ++cnt;
            }
        }
        if (cnt == n-1) break;
    }
    cout << ans << "\n";
}
