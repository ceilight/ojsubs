#include <cstdio>
#include <algorithm>
#include <vector>
#include <cmath>

using namespace std;

vector<int> graph[300100];
vector<vector<int>> queries;
int root[300100], compsize[300100];
int deg[300100], ancestor[300100][20];
int distime[300100], fintime[300100];
int timer;

int find(int u) {
    while (u != root[u]) u = root[u];
    return u;
}

bool unite(int u, int v) {
    u = find(u), v = find(v);
    if (u == v) return false;
    if (compsize[u] < compsize[v]) swap(u, v);
    compsize[u] += compsize[v];
    root[v] = u;
    return true;
}
 
void dfs(int u, int par) {
    distime[u] = ++timer;
    ancestor[u][0] = par;
    for (int k = 1; k <= 19; k++) {
        ancestor[u][k] = ancestor[ancestor[u][k-1]][k-1];
    }
    for (auto v : graph[u]) {
        if (v != par) dfs(v, u);
    }
    fintime[u] = ++timer;
}

bool is_ancestor(int u, int v) {
    return distime[u] <= distime[v] && fintime[v] <= fintime[u];
}

int lca(int u, int v) {
    if (is_ancestor(u, v)) return u;
    if (is_ancestor(v, u)) return v;
    for (int i = 19; i >= 0; i--) {
        if (!is_ancestor(ancestor[u][i], v))
            u = ancestor[u][i];
    }
    return ancestor[u][0];
}

vector<int> get_path(int u, int v) {
    vector<int> upath, vpath;

    int mid_node = lca(u, v);
    while (u != mid_node) {
        upath.push_back(u), u = ancestor[u][0];
    }
    while (v != mid_node) {
        vpath.push_back(v), v = ancestor[v][0];
    }

    upath.push_back(mid_node);
    for (int i = int(vpath.size())-1; i >= 0; i--)
        upath.push_back(vpath[i]);
    return upath;
}

int main() {
    int n, m;
    scanf("%d%d", &n, &m);
    
    for (int i = 1; i <= n; i++)
        root[i] = i, compsize[i] = 1;
    for (int i = 0; i < m; i++) {
        int u, v;
        scanf("%d%d", &u, &v);
        if (unite(u, v)) {
            graph[u].push_back(v);
            graph[v].push_back(u);
        }
    }

    dfs(1, 0);
    fintime[0] = ++timer;
    
    int q;
    scanf("%d", &q);
    for (int i = 0; i < q; i++) {
        int u, v;
        scanf("%d%d", &u, &v);
        queries.push_back(get_path(u, v));
        deg[u]++, deg[v]++;
    }

    int odddeg_count = 0;
    for (int i = 1; i <= n; i++) {
        if (deg[i] & 1) odddeg_count++;
    }
    if (odddeg_count) {
        printf("NO\n%d\n", odddeg_count/2);
    } else {
        puts("YES");
        for (auto path : queries) {
            printf("%d\n", path.size());
            for (auto node : path) printf("%d ", node);
            puts("");
        }
    }
    return 0;
}