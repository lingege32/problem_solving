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
    static vector<int> maximumBeauty(vector<vector<int>>& items, vector<int>& queries) {
        map<int, int> prices;
        for (auto& item : items) {
            auto it = prices.upper_bound(item[0]);
            if (it == prices.begin()) {
                prices[item[0]] = item[1];
                it = prices.find(item[0]);
                ++it;
                while (it != prices.end() && it->second <= item[1]) {
                    it = prices.erase(it);
                }
            } else {
                --it;
                if (it->second < item[1]) {
                    prices[item[0]] = item[1];
                    it = prices.find(item[0]);
                    ++it;
                    while (it != prices.end() && it->second <= item[1]) {
                        it = prices.erase(it);
                    }
                }
            }
        }

        for (auto& q : queries) {
            auto it = prices.upper_bound(q);
            q = it == prices.begin() ? 0 : (--it)->second;
        }
        return queries;
    }
};