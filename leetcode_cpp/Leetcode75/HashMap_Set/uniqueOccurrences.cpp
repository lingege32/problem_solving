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
    static bool uniqueOccurrences(vector<int>& arr) {
        std::unordered_map<int, int> count;
        for (auto a : arr) {
            count[a]++;
        }
        std::unordered_set<int> c;
        for (auto [k, v] : count) {
            auto [it, suc] = c.insert(v);
            if (!suc) {
                return false;
            }
        }
        return true;
    }
};