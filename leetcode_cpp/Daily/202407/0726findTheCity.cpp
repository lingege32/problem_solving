#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class OptimizeSolution {
  public:
    static int findTheCity(int n, vector<vector<int>>& edges, int th) {
        vector<vector<int>> dp(n, vector<int>(n, 1e9));
        for (auto& it : edges) {
            int i = it[0];
            int j = it[1];
            int w = it[2];
            dp[i][j] = w;
            dp[j][i] = w;
        }
        
        for (int via = 0; via < n; via++) {
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < n; j++) {
                    dp[i][j] = std::min(dp[i][via] + dp[via][j], dp[i][j]);
                }
            }
        }
        int ans = 0;
        int minCnt = 1e9;
        for (int i = 0; i < n; i++) {
            int cnt = 0;
            for (int j = 0; j < n; j++) {
                if (i != j) {
                    if (dp[i][j] <= th) {
                        cnt++;
                    }
                }
            }
            if (cnt <= minCnt) {
                minCnt = cnt;
                ans = i;
            }
        }
        return ans;
    }
};

class Solution {
  public:
    static int findTheCity(int n, vector<vector<int>>& edges, int distanceThreshold) {
        std::vector<std::vector<std::pair<int, int>>> costs;
        costs.resize(n);
        for (const auto& edge : edges) {
            auto n1 = edge[0];
            auto n2 = edge[1];
            auto c = edge[2];
            costs[n1].emplace_back(n2, c);
            costs[n2].emplace_back(n1, c);
        }
        int minCost = INT_MAX;
        int ret = -1;
        for (int i = 0; i < n; ++i) {
            std::vector<int> dist(n, INT_MAX);
            dist[i] = 0;
            std::queue<int> q;
            q.push(i);
            while (!q.empty()) {
                auto node = q.front();
                q.pop();
                auto cur = dist[node];
                auto& adjEdges = costs[node];
                for (auto [j, weight] : adjEdges) {
                    auto next = cur + weight;
                    if (next > distanceThreshold) {
                        continue;
                    }
                    if (next < dist[j]) {
                        dist[j] = next;
                        q.push(j);
                    }
                }
            }
            auto c = std::count_if(dist.begin(), dist.end(), [&](int d) { return d <= distanceThreshold; }) - 1;
            if (c <= minCost) {
                minCost = c;
                ret = i;
            }
        }
        return ret;
    }
};