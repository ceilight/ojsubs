#include <bits/stdc++.h>

using namespace std;

struct Edge {
    int u, v, w, id;
};

Edge edges[200100];
vector<int> adj[200100];
vector<Edge> mst;
int n, m, par[200100], rnk[200100], anc[200100][20], emx[200100][20], dep[200100];
long long tot, ans[200100];

bool cmp(Edge a, Edge b) {
    return a.w < b.w;
}

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

void kruskal() {
    sort(edges, edges+m, cmp);
    for (int i = 1; i <= n; i++) par[i] = i, rnk[i] = 1;
    for (int i = 0; i < m; i++) {
        Edge e = edges[i];
        if (unite(e.u, e.v)) {
            adj[e.u].push_back(e.v);
            adj[e.v].push_back(e.u);
            mst.push_back(e);
            tot += e.w;
        }
    }
}

void dfs(int u) {
    for (int v : adj[u]) if (v != anc[u][0]) {
        anc[v][0] = u;
        dep[v] = dep[u]+1;
        dfs(v);
    }
}

int heaviest(int u, int v) {
    if (u == anc[v][0]) return emx[v][0];
    if (v == anc[u][0]) return emx[u][0];
    int mx = 0;
    if (dep[u] < dep[v]) swap(u, v);
    for (int i = 19; i >= 0; i--) {
        if (dep[anc[u][i]] >= dep[v]) {
            mx = max(mx, emx[u][i]), u = anc[u][i];
        }
    }
    if (u == v) return mx;
    for (int i = 19; i >= 0; i--) {
        if (anc[u][i] != anc[v][i]) {
            mx = max({mx, emx[u][i], emx[v][i]});
            u = anc[u][i], v = anc[v][i];
        }
        if (u == v) break;
    }
    mx = max({mx, emx[u][0], emx[v][0]});
    return mx;
}

int main() {
    ios_base::sync_with_stdio(false); cin.tie(nullptr);
    
    cin >> n >> m;
    for (int i = 0; i < m; i++) {
        int u, v, w;
        cin >> u >> v >> w;
        edges[i] = {u, v, w, i};
    }
    kruskal();
    anc[1][0] = 0, emx[1][0] = 0, dep[1] = 0;
    dfs(1);
    for (int i = 0; i < n-1; i++) {
        Edge e = mst[i];
        if (dep[e.u] < dep[e.v]) swap(e.u, e.v);
        emx[e.u][0] = e.w;
    }
    for (int k = 1; k < 20; k++)
        for (int i = 1; i <= n; i++) {
            anc[i][k] = anc[anc[i][k-1]][k-1];
            emx[i][k] = max(emx[i][k-1], emx[anc[i][k-1]][k-1]);
        }
    for (int i = 0; i < m; i++) {
        Edge e = edges[i];
        ans[e.id] = tot-heaviest(e.u, e.v)+e.w;
    }
    for (int i = 0; i < m; i++) cout << ans[i] << "\n";
    return 0;
}