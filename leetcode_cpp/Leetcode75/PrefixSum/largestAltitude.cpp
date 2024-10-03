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
    static int largestAltitude(vector<int>& gain) {
        int max = gain[0];
        int n = gain.size();
        for (int i = 1; i < n; i++) {
            auto& g = gain[i];
            g += gain[i - 1];
            max = std::max(max, g);
        }
        return std::max(0, max);
    }
};