#include <bits/stdc++.h>

#include <algorithm>
#include <ranges>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    using int2 = pair<int, int>;

    static vector<int> leftmostBuildingQueries(vector<int>& heights, vector<vector<int>>& queries) {
        int n = heights.size(), qz = queries.size();
        std::vector<int> ret(qz, -1);
        std::vector<int2> idx;
        for (int i = 0; i < qz; ++i) {
            auto& alice_idx = queries[i][0];
            auto& bob_idx = queries[i][1];
            if (alice_idx > bob_idx) {
                std::swap(alice_idx, bob_idx);
            }
            if (alice_idx == bob_idx || heights[alice_idx] < heights[bob_idx]) {
                ret[i] = bob_idx;
            } else {
                idx.emplace_back(bob_idx, i);
            }
        }

        std::ranges::sort(idx, std::greater<>());

        std::vector<int2> stack;
        int j = n - 1;
        for (auto& [bob_idx, i] : idx) {
            int alice_idx = queries[i][0];
            for (; j > bob_idx; --j) {
                while (!stack.empty() && stack.back().first <= heights[j]) {
                    stack.pop_back();
                }
                stack.emplace_back(heights[j], j);
            }
            auto alice_height = heights[alice_idx];
            auto it = std::ranges::upper_bound(std::ranges::reverse_view(stack), make_pair(alice_height, n));
            if (it != stack.rend()) {
                ret[i] = it->second;
            }
        }
        return ret;
    }
};
