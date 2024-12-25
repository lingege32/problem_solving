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
    static int minimumDiameterAfterMerge(const vector<vector<int>>& edges1, const vector<vector<int>>& edges2) {
        auto d1 = getDiameter(edges1);
        auto d2 = getDiameter(edges2);
        return std::max({d1, d2, 1 + ((d1 + 1) / 2) + ((d2 + 1) / 2)});
    }

  private:
    static int getDiameter(const vector<vector<int>>& edges) {
        const int n = edges.size() + 1;
        vector<vector<int>> graph(n);

        for (const vector<int>& edge : edges) {
            const int u = edge[0];
            const int v = edge[1];
            graph[u].push_back(v);
            graph[v].push_back(u);
        }

        int maxDiameter = 0;
        maxDepth(graph, 0, /*prev=*/-1, maxDiameter);
        return maxDiameter;
    }

    static int maxDepth(const vector<vector<int>>& graph, int u, int prev, int& maxDiameter) {
        int maxSubDepth1 = 0;
        int maxSubDepth2 = 0;
        for (const int v : graph[u]) {
            if (v == prev) {
                continue;
            }
            const int maxSubDepth = maxDepth(graph, v, u, maxDiameter);
            if (maxSubDepth > maxSubDepth1) {
                maxSubDepth2 = maxSubDepth1;
                maxSubDepth1 = maxSubDepth;
            } else if (maxSubDepth > maxSubDepth2) {
                maxSubDepth2 = maxSubDepth;
            }
        }
        maxDiameter = max(maxDiameter, maxSubDepth1 + maxSubDepth2);
        return 1 + maxSubDepth1;
    }
};