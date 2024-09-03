#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    int maxProfitAssignment(vector<int> &difficulty, vector<int> &profit,
                            vector<int> &worker) {
        std::sort(worker.begin(), worker.end());
        size_t n = difficulty.size();
        std::vector<pair<int, int>> p;
        p.reserve(n);
        for (size_t i = 0; i < n; ++i) {
            p.emplace_back(difficulty[i], profit[i]);
        }
        std::sort(p.begin(), p.end());
        int max_profit = 0;
        size_t idx = 0;
        int ret = 0;
        for (auto w : worker) {
            while (idx < n && p[idx].first <= w) {
                max_profit = std::max(max_profit, p[idx].second);
                idx++;
            }
            ret += max_profit;
        }
        return ret;
    }
};
