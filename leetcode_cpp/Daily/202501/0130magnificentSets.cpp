#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static int bfs(int start, const std::vector<std::vector<int>>& graph, std::vector<int>& level) {
        if (graph[start].empty()) {
            level[start] = 1;
            return 1;
        }
        std::vector<int> cur, next;
        cur.push_back(start);
        int l = 0;
        while (!cur.empty()) {
            for (auto node : cur) {
                auto& nl = level[node];
                if (nl != -1) {
                    if (nl != l && nl + 2 != l) {
                        return -1;
                    }
                    continue;
                }
                nl = l;
                for (auto next_node : graph[node]) {
                    next.push_back(next_node);
                }
            }
            cur.swap(next);
            next.clear();
            l++;
        }
        return l - 1;
    }

    static int magnificentSets(int n, vector<vector<int>>& edges) {
        std::vector<std::vector<int>> graph(n + 1);
        for (auto& edge : edges) {
            graph[edge[0]].push_back(edge[1]);
            graph[edge[1]].push_back(edge[0]);
        }
        std::vector<int> parents(n + 1, -1);
        std::vector<int> level(n + 1, -1);
        std::vector<int> levelize(n + 1, -1);
        for (int i = 1; i <= n; ++i) {
            level.assign(n + 1, -1);
            int l = bfs(i, graph, level);
            if (l == -1) {
                return -1;
            }

            if (parents[i] == -1) {
                for (int child = 1; child <= n; ++child) {
                    if (level[child] != -1) {
                        parents[child] = i;
                    }
                }
            }

            levelize[parents[i]] = std::max(levelize[parents[i]], l);
        }
        int ret = 0;
        for (auto l : levelize) {
            if (l != -1) {
                ret += l;
            }
        }
        return ret;
    }
};

class OptSolution {
  public:
    static int magnificentSets(int n, vector<vector<int>>& edges) {
        auto adj = vector<vector<int>>(n);
        for (const auto& edge : edges) {
            auto [u, v] = make_pair(edge.front(), edge.back());
            adj[u - 1].push_back(v - 1);
            adj[v - 1].push_back(u - 1);
        }

        auto node_color = vector<int>(n, uncolored);
        auto ccs = vector<vector<int>>{};
        for (auto u : views::iota(0, n)) {
            if (node_color[u] != uncolored) {
                continue;
            }
            if (!dfs(u, adj, black, node_color, ccs.emplace_back())) {
                return -1;
            }
        }

        return transform_reduce(begin(ccs), end(ccs), 0, plus{},
                                [&adj](const auto& cc) { return max_groups_for_cc(cc, adj); });
    }

  private:
    static constexpr auto uncolored = -1;
    static constexpr auto black = 0;
    static constexpr auto white = 1;

    // dfs to mark the color to check if the adj is ok 
    static bool dfs(int u, const vector<vector<int>>& adj, int color, vector<int>& node_color, vector<int>& cc) {
        auto complement = !color;
        node_color[u] = color;
        cc.push_back(u);
        for (auto v : adj[u]) {
            if (node_color[v] == color || (node_color[v] == uncolored && !dfs(v, adj, complement, node_color, cc))) {
                return false;
            }
        }
        return true;
    }
    // bfs to check the maximum groups
    static int max_groups_for_cc(const vector<int>& cc, const vector<vector<int>>& adj) {
        if (size(cc) == 1) {
            return 1;
        }

        auto q = vector<int>{};
        auto next_q = vector<int>{};
        auto max_groups = 0;
        for (auto start : cc) {
            q.push_back(start);
            auto seen = bitset<500>{1} << start;
            auto groups = 0;
            while (!empty(q)) {
                ++groups;
                for (auto u : q) {
                    for (auto v : adj[u]) {
                        if (seen[v]) {
                            continue;
                        }
                        seen.set(v);
                        next_q.push_back(v);
                    }
                }
                swap(q, next_q);
                next_q.clear();
            }
            max_groups = std::max(groups, max_groups);
        }
        return max_groups;
    }
};