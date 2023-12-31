#include <algorithm>
#include <cassert>
#include <iostream>
#include <limits>
#include <map>
#include <queue>
#include <sstream>
#include <string>
#include <vector>

template <class flow_t = int64_t>
struct Dinitz {
    struct edge_t {
        int v, index;
        flow_t capacity;
    };

    int n;
    std::vector<std::vector<edge_t>> adj;

    Dinitz(int n): n(n) {
        adj.resize(n);
    }

    std::pair<int, int> add_edge(int u, int v, flow_t capacity, flow_t reverse_capacity = 0) {
        assert(capacity >= 0 && reverse_capacity >= 0);
        edge_t uv = {v, (int)adj[v].size() + u == v ? 1 : 0, capacity};
        edge_t vu = {u, (int)adj[u].size(), reverse_capacity};
        adj[u].push_back(uv);
        adj[v].push_back(vu);
        return std::make_pair(u, (int)adj[u].size() - 1);
    }

    flow_t get_flow(std::pair<int, int> edge) {
        const edge_t &e = adj[edge.first][edge.second];
        return adj[e.v][e.index].capacity;
    }

    std::vector<int> level, pointer;

    bool bfs(int s, int t) {
        level.assign(n, 0);
        level[s] = 1;
        std::queue<int> q;
        q.push(s);
        while (!q.empty()) {
            int u = q.front();
            q.pop();
            for (auto &e : adj[u]) {
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
        while (pointer[u] < (int)adj[u].size()) {
            edge_t &e = adj[u][pointer[u]];
            if (e.capacity > 0 && level[e.v] == level[u] + 1) {
                flow_t next_flow = dfs(e.v, t, std::min(current_flow, e.capacity));
                if (next_flow > 0) {
                    e.capacity -= next_flow;
                    adj[e.v][e.index].capacity += next_flow;
                    return next_flow;
                }
            }
            pointer[u]++;
        }
        return 0;
    }

    flow_t compute_flow(int s, int t) {
        flow_t flow = 0;
        while (bfs(s, t)) {
            pointer.assign(n, 0);
            flow_t next_flow;
            do {
                next_flow = dfs(s, t, std::numeric_limits<flow_t>::max());
                flow += next_flow;
            } while (next_flow > 0);
        }
        return flow;
    }

    std::vector<std::pair<int, int>> compute_cut(std::vector<std::pair<int, int>> edges) {
        std::vector<std::pair<int, int>> answer;
        for (auto &[u, index] : edges) {
            auto e = adj[u][index];
            if (level[u] != 0 && level[e.v] == 0 && e.capacity == 0) {
                answer.emplace_back(u, e.v);
            }
        }
        return answer;
    }
};

int main(void) {
    std::ios::sync_with_stdio(false);

    // Parse input
    std::string line;
    std::map<std::string, std::vector<std::string>> adj;
    std::map<std::string, int> indices;
    int nnode = 0;

    while (std::getline(std::cin, line)) {
        std::stringstream split(line);
        std::string temp;
        split >> temp;
        std::string src = temp.substr(0, temp.size() - 1);
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

    // Solution approach:
    // Apply Dinitz's algorithm to calculate flow in the graph until a source/sink vertice
    // with a max flow of 3 is found
    // Remove any edges used in those flows
    // BFS to get the size of one of the components then multiply with the size of the rest
    Dinitz network(nnode);
    std::vector<std::vector<int>> graph(nnode);
    std::vector<std::pair<int, int>> edges;

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

    for (auto [u, v] : cut) {
        auto remv = std::remove(graph[u].begin(), graph[u].end(), v);
        auto remu = std::remove(graph[v].begin(), graph[v].end(), u);
        graph[u].erase(remv, graph[u].end());
        graph[v].erase(remu, graph[v].end());
    }

    int ncomp = 0;
    std::vector<bool> visited(nnode, false);
    std::queue<int> q;
    q.push(0);
    while (!q.empty()) {
        int cur = q.front();
        q.pop();
        if (visited[cur]) {
            continue;
        }
        ncomp++;
        visited[cur] = true;
        for (auto &dst : graph[cur]) {
            q.push(dst);
        }
    }

    std::cout << "Part 1: " << ncomp * (nnode - ncomp) << std::endl;
}
