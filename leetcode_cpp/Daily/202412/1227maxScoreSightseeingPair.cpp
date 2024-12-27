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
    static int maxScoreSightseeingPair(vector<int>& values) {
        int n = values.size();
        int max = values[0];
        int ret = std::numeric_limits<int>::min();
        for (int i = 1; i < n; ++i) {
            int v = values[i];
            ret = std::max(ret, max + v - i);
            max = std::max(max, values[i] + i);
        }
        return ret;
    }
};