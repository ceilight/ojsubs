// https://oj.vnoi.info/problem/khistory

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 100100

int min(int a, int b) {
    return a < b ? a : b;
}

int par[N], siz[N];
int vis[N], cnt, ti[N], lo[N], *gv[N], *gk[N], deg[N], bridge[N];
int group[N], *gv2[N], deg2[N], f[N][3];

int root(int a) {
    return par[a] == a ? a : (par[a] = root(par[a]));
}
 
void unite(int u, int v) {
    int tmp;
    
    u = root(u), v = root(v);
    if (u == v) {
        return;
    }
    if (siz[u] < siz[v]) {
        tmp = u;
        u = v;
        v = tmp;
    }
    siz[u] += siz[v];
    par[v] = u;
}

void add_edge(int u, int v, int k) {
    int h = deg[u]++;
    
    if (h >= 2 && (h & h-1) == 0) {
        gv[u] = realloc(gv[u], h * 2 * sizeof(*gv[u]));
        gk[u] = realloc(gk[u], h * 2 * sizeof(*gk[u]));
    }
    gv[u][h] = v, gk[u][h] = k;
}

void dfs(int u, int p) {
    int v, k, i;
    
    vis[u] = 1;
    ti[u] = lo[u] = cnt++;
    for (i = 0; i < deg[u]; i++) {
        v = gv[u][i], k = gk[u][i];
        if (v != p) {
            if (vis[v]) {
                lo[u] = min(lo[u], ti[v]);
            } else {
                dfs(v, u);
                lo[u] = min(lo[u], lo[v]);
                if (lo[v] > ti[u]) {
                    bridge[k] = 1;
                }
            }
        }
    }
}

void add_bridge(int u, int v) {
    int h = deg2[u]++;
    
    if (h >= 2 && (h & h-1) == 0) {
        gv2[u] = realloc(gv2[u], h * 2 * sizeof(*gv2[u]));
    }
    gv2[u][h] = v;
}

void dfs2(int u, int p) {
    int t = 0, v, i;
    
    vis[u] = 1;
    f[u][0] = N, f[u][1] = 1, f[u][2] = 0;

    for (i = 0; i < deg2[u]; i++) {
        v = gv2[u][i];
        if (v != p) {
            dfs2(v, u);
            f[u][2] += f[v][0];
            f[u][1] += min(f[v][0], min(f[v][1], f[v][2]));
            t += min(f[v][0], f[v][1]);
        }
    }
    for (i = 0; i < deg2[u]; i++) {
        v = gv2[u][i];
        if (v != p) {
            f[u][0] = min(f[u][0], t - min(f[v][0], f[v][1]) + f[v][1]);
        }
    }
}

int main() {
    int n, m, i, res, u, v, k;
    
    scanf("%d%d", &n, &m);
    for (u = 1; u <= n; u++) {
        deg[u] = 0;
        gv[u] = malloc(2 * sizeof(*gv[u]));
        gk[u] = malloc(2 * sizeof(*gk[u]));
    }
    for (i = 0; i < m; i++) {
        scanf("%d%d", &u, &v);
        add_edge(u, v, i);
        add_edge(v, u, i);
    }

    memset(vis, 0, (n+1) * sizeof(int));
    for (u = 1; u <= n; u++) {
        if (!vis[u]) {
            dfs(u, -1);
        }
    }
    for (u = 1; u <= n; u++) {
        par[u] = u, siz[u] = 1;
    }
    for (u = 1; u <= n; u++) {
        for (i = 0; i < deg[u]; i++) {
            v = gv[u][i], k = gk[u][i];
            if (!bridge[k]) {
                unite(u, v);
            }
        }
    }

    cnt = 0;
    for (u = 1; u <= n; u++) {
        if (u == root(u)) {
            group[u] = ++cnt;
        }
    }
    for (u = 1; u <= n; u++) {
        if (u != root(u)) {
            group[u] = group[root(u)];
        }
    }
    for (u = 1; u <= cnt; u++) {
        deg2[u] = 0;
        gv2[u] = malloc(2 * sizeof(*gv2[u]));
    }

    for (u = 1; u <= n; u++) {
        for (i = 0; i < deg[u]; i++) {
            v = gv[u][i], k = gk[u][i];
            if (u < v && bridge[k]) {
                add_bridge(group[u], group[v]);
                add_bridge(group[v], group[u]);
            }
        }
    }
    
    res = 0;
    memset(vis, 0, (cnt+1) * sizeof(*vis));
    for (u = 1; u <= cnt; u++) {
        if (!vis[u]) {
            dfs2(u, -1);
            res += min(f[u][0], f[u][1]);
        }
    }
    printf("%d\n", res);
    return 0;
}
