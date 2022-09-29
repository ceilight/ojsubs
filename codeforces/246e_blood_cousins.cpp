#include <bits/stdc++.h>
using namespace std;

const int nmax = 100100;

struct Query {
    int idx, k;
};

int par[nmax], siz[nmax];
deque<set<string>> st[nmax];

int root(int a) {
    while (par[a] != a) {
        a = par[a];
    }
    return par[a];
}

bool unite(int u, int v) {
    u = root(u), v = root(v);
    if (u == v) return false;
    if (siz[u] < siz[v]) {
        swap(u, v);
    }
    siz[u] += siz[v];
    par[v] = u;

    // move set root(v) to set root(u)
    // common depths
    for (int i = 0; i < st[u].size() and i < st[v].size(); i++) {
        for (auto j : st[v][i]) {
            st[u][i].insert(j);
        }
    }
    // if subtree v has more depth than u
    int len = st[u].size();
    for (int i = len; i < st[v].size(); i++) {
        st[u].push_back(st[v][i]);
    }
    return true;
}

char buf[10100];

vector<int> adj[nmax];
bool is_root[nmax];
string name[nmax];
vector<Query> que[nmax];
int ans[nmax];

void dfs(int u, int par) {
    for (auto v : adj[u]) {
        if (v != par) {
            dfs(v, u);
            unite(v, u);
        }
    }

    // add name of node u to set of root(u)
    int uu = root(u);
    set<string> temp;
    temp.insert(name[u]);
    st[uu].push_front(temp);
    siz[uu]++;

    // query
    for (auto q : que[u]) {
        if (q.k >= st[uu].size()) {
            ans[q.idx] = 0; 
        }
        else {
            ans[q.idx] = st[uu][q.k].size();
        }
    }
}

int main() {
    int n;
    scanf("%d", &n);
    for (int i = 1; i <= n; i++) {
        int par;
        memset(buf, 0, sizeof buf);
        scanf("%s%d", buf, &par);
        name[i] = string(buf);
        if (par) {
            adj[par].push_back(i);
        }
        else {
            is_root[i] = true;
        }
    }

    int m;
    scanf("%d", &m);
    for (int i = 1; i <= m; i++) {
        int v, k;
        scanf("%d%d", &v, &k);
        que[v].push_back({i, k});
    }

    // union-find init
    for (int i = 1; i <= n; i++) {
        par[i] = i;
    }

    for (int i = 1; i <= n; i++) {
        if (is_root[i]) {
            dfs(i, -1);
        }
    }
    for (int i = 1; i <= m; i++) {
        printf("%d\n", ans[i]);
    }
    return 0;
}