#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int maxScore(vector<vector<int>>& grid) {
        for (auto& g : grid) {
            std::sort(g.begin(), g.end());
            g.erase(std::unique(g.begin(), g.end()), g.end());
        }

        std::vector<int> stack;
        stack.reserve(grid.size());
        int max = 0;
        auto gl = grid.size();
        std::function<void(size_t)> dfs = [&](size_t level) {
            if (level == gl) {
                max = std::max(max, std::accumulate(stack.begin(), stack.end(), 0));
                return;
            }
            dfs(level + 1);
            for (auto v : grid[level]) {
                auto it = std::find(stack.begin(), stack.end(), v);
                if (it == stack.end()) {
                    stack.push_back(v);
                    dfs(level + 1);
                    stack.pop_back();
                }
            }
        };
        dfs(0);
        return max;
    }
};