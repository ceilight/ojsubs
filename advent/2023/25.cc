#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <limits>
#include <map>
#include <queue>
#include <sstream>
#include <string>
#include <vector>

using namespace std;

template <class flow_t = int64_t>
struct Dinitz {
    struct Edge {
        int v, index;
        flow_t capacity;
    };

    int n;
    vector<vector<Edge>> a;

    Dinitz(int n): n(n) {
        a.resize(n);
    }

    pair<int, int> add_edge(int u, int v, flow_t capacity, flow_t reverse_capacity = 0) {
        assert(min(capacity, reverse_capacity) >= 0);
        a[u].push_back({v, (int)a[v].size(), capacity});
        a[v].push_back({u, (int)a[u].size() - 1, reverse_capacity});
        return make_pair(u, (int)a[u].size() - 1);
    }

    flow_t get_flow(pair<int, int> edge) {
        const Edge &e = a[edge.first][edge.second];
        return a[e.v][e.index].capacity;
    }

    vector<int> level, pointer;

    bool bfs(int s, int t) {
        level = pointer = vector<int>(n);
        level[s] = 1;
        queue<int> q;
        q.push(s);
        while (!q.empty()) {
            int u = q.front();
            q.pop();
            for (auto &e : a[u]) {
                if (e.capacity > 0 && level[e.v] == 0) {
                    q.push(e.v);
                    level[e.v] = level[u] + 1;
                    if (e.v == t) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    flow_t dfs(int u, int t, flow_t current_flow) {
        if (u == t) {
            return current_flow;
        }
        for (int& i = pointer[u]; i < (int)a[u].size(); ++i) {
            Edge& e = a[u][i];
            if (level[e.v] != level[u] + 1 || e.capacity == 0) {
                continue;
            }
            flow_t next_flow = dfs(e.v, t, min(current_flow, e.capacity));
            if (next_flow > 0) {
                e.capacity -= next_flow;
                a[e.v][e.index].capacity += next_flow;
                return next_flow;
            }
        }
        return 0;
    }

    flow_t compute_flow(int s, int t) {
        flow_t flow = 0;
        while (bfs(s, t)) {
            flow_t next_flow;
            do {
                next_flow = dfs(s, t, numeric_limits<flow_t>::max());
                flow += next_flow;
            } while (next_flow > 0);
        }
        return flow;
    }

    vector<pair<int, int>> compute_cut(vector<pair<int, int>> edges) {
        vector<pair<int, int>> answer;
        for (auto& [u, index] : edges) {
            auto e = a[u][index];
            if (level[u] != 0 && level[e.v] == 0 && e.capacity == 0) {
                answer.emplace_back(u, e.v);
            }
        }
        return answer;
    }
};

int main(void) {
    ios::sync_with_stdio(false);

    string line;
    map<string, vector<string>> adj;
    map<string, int> indices;
    int nnode = 0;

    while (getline(cin, line)) {
        stringstream split(line);
        string temp;
        split >> temp;
        string src = temp.substr(0, temp.size() - 1);
        if (indices.find(src) == indices.end()) {
            indices[src] = nnode++;
        }
        while (split >> temp) {
            adj[src].push_back(temp);
            if (indices.find(temp) == indices.end()) {
                indices[temp] = nnode++;
            }
        }
    }

    Dinitz network(nnode);
    vector<vector<int>> graph(nnode);
    vector<pair<int, int>> edges;

    for (auto &[k, v] : adj) {
        int src = indices[k];
        for (auto &x : v) {
            int dst = indices[x];
            graph[src].push_back(dst);
            graph[dst].push_back(src);
            edges.push_back(network.add_edge(src, dst, 1));
            edges.push_back(network.add_edge(dst, src, 1));
        }
    }

    auto flow = network.compute_flow(0, 3);
    auto cut = network.compute_cut(edges);

    for (auto [src, dst] : cut) {
        graph[src].erase(remove(graph[src].begin(), graph[src].end(), dst), graph[src].end());
        graph[dst].erase(remove(graph[dst].begin(), graph[dst].end(), src), graph[dst].end());
    }

    queue<int> q;
    int ncomp = 0;
    vector<bool> visited(nnode, false);
    q.push(0);
    while (!q.empty()) {
        int cur = q.front();
        q.pop();
        if (visited[cur]) {
            continue;
        }
        ncomp++;
        visited[cur] = true;
        for (auto& dst : graph[cur]) {
            q.push(dst);
        }
    }

    cout << "Part 1: " << ncomp * (nnode - ncomp) << endl;
}
