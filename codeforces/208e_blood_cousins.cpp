#include <iostream>
#include <cmath>
#include <vector>
#include <algorithm>

using namespace std;

struct interval {
    int index, discover, finish;
    bool operator <(const interval &other) {
        return discover < other.discover;
    }
};

vector<int> graph[100100];
vector<interval> events[100100];
int ancestor[100100][20], depth[100100];
int distime[100100], fintime[100100];
int timer, LOG;

void dfs(int u, int p) {
    ++timer;
    distime[u] = timer;
    for (int k = 1; k <= LOG; k++) {
        ancestor[u][k] = ancestor[ancestor[u][k-1]][k-1];
    }
    for (auto v : graph[u]) {
        if (v != p) {
            depth[v] = depth[u]+1;
            dfs(v, u);
        }
    }
    fintime[u] = timer;
}

int get_ancestor(int u, int k) {
    for (int i = LOG; i >= 0; i--) {
        if (k & (1<<i)) u = ancestor[u][i];
    }
    return u;
}

int solve(int v, int p, vector<interval> &evt) {
    int p_ancestor = get_ancestor(v, p);
    int low, high, left, right;

    low = 0;
    high = left = evt.size()-1;
    while (low <= high) {
        int mid = low + high >> 1;
        if (evt[mid].discover >= distime[p_ancestor]) {
            high = mid-1, left = mid;
        } else {
            low = mid+1;
        }
    } 

    low = right = 0;
    high = evt.size()-1;
    while (low <= high) {
        int mid = low + high >> 1;
        if (evt[mid].finish <= fintime[p_ancestor]) {
            low = mid+1, right = mid;
        } else {
            high = mid-1;
        }
    }
    return right - left;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;

    LOG = ceil(log2(n)); // lol
    for (int k = 1; k <= LOG; k++) {
        for (int i = 1; i <= n; i++)
            ancestor[i][k] = -1;
    }

    for (int i = 1; i <= n; i++) {
        int p;
        cin >> p;
        if (p == 0) {
            ancestor[i][0] = -1;
            depth[i] = 1;
        } else {
            graph[p].push_back(i);
            ancestor[i][0] = p;
        }
    }
    for (int i = 1; i <= n; i++) {
        if (ancestor[i][0] == -1) dfs(i, -1);
    }
    for (int i = 1; i <= n; i++) {
        events[depth[i]].push_back({i, distime[i], fintime[i]});
    }
    for (int d = 1; d <= n; d++) { 
        sort(events[d].begin(), events[d].end());
    }

    int m;
    cin >> m;
    for (int query = 0; query < m; query++) {
        int v, p;
        cin >> v >> p;
        if (depth[v] <= p) {
            cout << "0 ";
        } else {
            cout << solve(v, p, events[depth[v]]) << " ";
        }
    }

    return 0;
}