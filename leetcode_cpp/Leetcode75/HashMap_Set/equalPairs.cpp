#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct VectorHasher {
    int operator()(const vector<int>& V) const {
        int hash = V.size();
        for (auto& i : V) {
            hash ^= i + 0x9e3779b9 + (hash << 6) + (hash >> 2);
        }
        return hash;
    }
};

class Solution {
  public:
    static int equalPairs(vector<vector<int>>& grid) {
        std::unordered_map<std::vector<int>, int, VectorHasher> mp;
        for (const auto& row : grid) {
            mp[row]++;
        }

        int ans = 0;
        int n = grid.size();
        for (int i = 0; i < n; ++i) {
            std::vector<int> col;
            col.reserve(n);
            for (int j = 0; j < n; ++j) {
                col.push_back(grid[j][i]);
            }
            auto it = mp.find(col);
            if (it != mp.end()) {
                ans += it->second;
            }
        }
        return ans;
    }
};