#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct hash_vector {
    std::size_t operator()(std::vector<int> const& vec) const {
        std::size_t seed = vec.size();
        for (auto& i : vec) {
            seed ^= i + 0x9e3779b9 + (seed << 6) + (seed >> 2);
        }
        return seed;
    }
};

class Solution {
  public:
    static int maxEqualRowsAfterFlips(vector<vector<int>>& matrix) {
        std::unordered_map<std::vector<int>, int, hash_vector> map;
        for (auto& row : matrix) {
            auto it = map.find(row);
            if (it != map.end()) {
                it->second++;
            } else {
                for (auto& v : row) {
                    v = 1 - v;
                }
                map[row]++;
            }
        }
        int max = 0;
        for (const auto& [k, v] : map) {
            max = std::max(max, v);
        }
        return max;
    }
};