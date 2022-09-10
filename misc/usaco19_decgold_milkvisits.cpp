// https://oj.vnoi.info/problem/usaco19dg_milkvisits

#include <bits/stdc++.h>
using namespace std;

const int nmax = 100100;
vector<int> adj[nmax];
int typ[nmax];
int par[nmax], dep[nmax], pos[nmax];
int root[nmax], heavy[nmax];
int n, m, idx;

int dfs(int u) {
    int tree_size = 1, max_sub_size = 0;
    for (auto v : adj[u]) {
        if (v != par[u]) {
            par[v] = u;
            dep[v] = dep[u]+1;
            int sub_size = dfs(v);
            tree_size += sub_size;
            if (max_sub_size < sub_size) {
                heavy[u] = v;
                max_sub_size = sub_size;
            }
        }
    }
    return tree_size;
}

void decompose(int u, int rt) {
    root[u] = rt;
    pos[u] = idx++;
    if (heavy[u] != 0) {
        decompose(heavy[u], rt);
    }
    for (auto v : adj[u]) {
        if (v != heavy[u] && v != par[u]) {
            decompose(v, v);
        }
    }
}

vector<int> seg[nmax<<1];

void seg_add(int pos, int val) {
    pos += n;
    seg[pos].push_back(val);
}

void seg_build() {
    for (int i = n-1; i > 0; i--) {
        seg[i].resize(seg[i<<1].size() + seg[i<<1|1].size());
        merge(seg[i<<1].begin(), seg[i<<1].end(),
            seg[i<<1|1].begin(), seg[i<<1|1].end(), seg[i].begin());
    }
}

int seg_query(int l, int r, int typ) {
    int cnt = 0;
    l += n, r += n;
    while (l < r) {
        if (l & 1) {
            cnt += upper_bound(seg[l].begin(), seg[l].end(), typ)
                - lower_bound(seg[l].begin(), seg[l].end(), typ);
            l++;
        }
        if (r & 1) {
            r--;
            cnt += upper_bound(seg[r].begin(), seg[r].end(), typ)
                - lower_bound(seg[r].begin(), seg[r].end(), typ);
        }
        l >>= 1, r >>= 1;
    }
    return cnt;
}

bool solve(int u, int v, int t) {
    int res = 0;
    while (root[u] != root[v]) {
        if (dep[root[u]] > dep[root[v]]) {
            swap(u, v);
        }
        res += seg_query(pos[root[v]], pos[v]+1, t);
        v = par[root[v]];
    }
    if (dep[u] > dep[v]) {
        swap(u, v);
    }
    res += seg_query(pos[u], pos[v]+1, t);
    return res > 0;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> n >> m;
    for (int i = 1; i <= n; i++) {
        cin >> typ[i];
    }
    for (int i = 1; i < n; i++) {
        int u, v;
        cin >> u >> v;
        adj[u].push_back(v);
        adj[v].push_back(u);
    }

    dfs(1);
    decompose(1, 1);

    for (int i = 1; i <= n; i++) {
        seg_add(pos[i], typ[i]);
    }
    seg_build();

    for (int i = 0; i < m; i++) {
        int a, b, c;
        cin >> a >> b >> c;
        cout << (int)solve(a, b, c);
    }
    return 0;
}
