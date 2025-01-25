#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static vector<int> eventualSafeNodes(vector<vector<int>>& graph) {
        int n = graph.size();
        std::vector<std::unordered_set<int>> edgein(n);
        std::vector<int> ret;
        size_t cur_size = 0;
        for (auto i = 0; i < n; ++i) {
            if (graph[i].empty()) {
                ret.push_back(i);
                continue;
            }
            for (auto out : graph[i]) {
                edgein[out].insert(i);
            }
        }
        for (; cur_size != ret.size();) {
            size_t s = ret.size();
            for (; cur_size < s; ++cur_size) {
                auto removed = ret[cur_size];
                for (auto in : edgein[removed]) {
                    graph[in].pop_back();
                    if (graph[in].empty()) {
                        ret.push_back(in);
                    }
                }
            }
        }
        std::ranges::sort(ret);
        return ret;
    }
};

class OptSolution {
  public:
    // Question is asking us to find nodes that are not in a cycle
    // any noode which is part of cycle is not safe
    // any node leading to a cycle is not safe
    bool dfs(int node, vector<vector<int>>& adj, vector<int>& vis, vector<int>& Pathvis, vector<int>& check) {
        vis[node] = 1;
        Pathvis[node] = 1;
        check[node] = 0;
        for (auto it : adj[node]) {
            if (!vis[it]) {
                vis[it] = 1;
                if (dfs(it, adj, vis, Pathvis, check)) {
                    check[node] = 0;
                    return true;
                }
            } else if (Pathvis[it]) {
                check[node] = 0;
                return true;
            }
        }
        check[node] = 1;
        Pathvis[node] = 0;
        return false;
    }

    vector<int> eventualSafeNodes(vector<vector<int>>& graph) {
        int V = graph.size();
        vector<int> vis(V, 0);
        vector<int> Pathvis(V, 0);
        vector<int> check(V, 0);
        vector<int> ans;
        for (int i = 0; i < V; i++) {
            if (!vis[i]) {
                dfs(i, graph, vis, Pathvis, check);
            }
        }
        for (int i = 0; i < V; i++) {
            if (check[i] == 1) {
                ans.push_back(i);
            }
        }
        return ans;
    }
};