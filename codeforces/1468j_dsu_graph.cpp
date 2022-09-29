#include <bits/stdc++.h>

#define fi first
#define se second
#define mp make_pair
#define all(a) (a).begin(), (a).end()
#define reset(a, b) memset(a, b, sizeof(a))
template<typename T> bool chkmax(T &a, T b) { 
    return a < b ? a = b, 1 : 0;
}
template<typename T> bool chkmin(T &a, T b) {
    return a > b ? a = b, 1 : 0;
}
using llong = long long;
using namespace std;

int parent[200100];
int sz[200100];

int find(int a) {
    if(a == parent[a]) return a;
    return parent[a] = find(parent[a]);
}

bool unite(int u, int v) {
    int i = find(u);
    int j = find(v);
    if(i == j) return false;
    if(sz[i] < sz[j]) swap(i, j);
    sz[i] += sz[j];
    parent[j] = i;
    return true;
}

void solve() {
    int n, m, k;
    scanf("%d%d%d", &n, &m, &k);
    vector<pair<int, pair<int, int>>> edge(m); 
    for(int i = 0; i < m; i++) {
        int u, v, s;
        scanf("%d%d%d", &u, &v, &s);
        edge[i] = mp(s, mp(--u, --v));
    }
    sort(all(edge));
    for(int i = 0; i < n; i++) {
        parent[i] = i;
        sz[i] = 1;
    }
    llong cost = 0;
    for(auto i : edge) {
        if(unite(i.se.fi, i.se.se))
            cost += 1LL * max(0, i.fi - k);
    }
    if(cost > 0) printf("%lld\n", cost);
    else {
        int least = 1e9 + 7;
        for(auto i : edge)
            chkmin(least, abs(i.fi - k));
        printf("%d\n", least);
    }
}

int main() {
    int t;
    scanf("%d", &t);
    while(t--)
        solve();
    return 0;
}