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
    static vector<int> finalPrices(vector<int>& prices) {
        const int n = prices.size();
        for (int i = 0; i < n - 1; ++i) {
            auto& v = prices[i];
            for (int j = i + 1; j < n; ++j) {
                if (prices[j] <= v) {
                    v -= prices[j];
                    break;
                }
            }
        }
        return std::move(prices);
    }
};