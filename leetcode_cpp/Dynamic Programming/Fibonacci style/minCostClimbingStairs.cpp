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
    static int minCostClimbingStairs(vector<int>& cost) {
        int n = cost.size();
        int prev1 = 0;
        int prev2 = 0;
        for (int i = 2; i < n + 1; ++i) {
            int min = std::min(prev1 + cost[i - 2], prev2 + cost[i - 1]);
            prev1 = prev2;
            prev2 = min;
        }
        return prev2;
    }
};